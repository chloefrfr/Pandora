use dashmap::DashMap;
use lazy_static::lazy_static;
use log::{debug, error, info, warn};
use packet_manager::PacketManager;
use rand::random;
use std::{
    io::Cursor,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
};
use structs::handshake_struct::HandshakePacket;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::{mpsc, Mutex},
};
use uuid::Uuid;

lazy_static! {
    pub static ref CONNECTION_MANAGER: ConnectionManager = ConnectionManager::new();
}

pub mod structs;

#[derive(Debug, Clone)]
pub struct Connection {
    pub id: u32,
    pub socket: Arc<Mutex<TcpStream>>,
    pub player_uuid: Option<Uuid>,
    pub send_queue: mpsc::Sender<Vec<u8>>,
    pub recv_queue: Arc<Mutex<mpsc::Receiver<Vec<u8>>>>,
}

impl Connection {
    pub fn new(
        socket: TcpStream,
        id: u32,
        player_uuid: Option<Uuid>,
    ) -> (
        Arc<Self>,
        mpsc::Sender<Vec<u8>>,
        Arc<Mutex<mpsc::Receiver<Vec<u8>>>>,
    ) {
        let (send_tx, _send_rx) = mpsc::channel(1024);
        let (_recv_tx, recv_rx) = mpsc::channel(1024);

        let connection = Connection {
            id,
            socket: Arc::new(Mutex::new(socket)),
            player_uuid,
            send_queue: send_tx.clone(),
            recv_queue: Arc::new(Mutex::new(recv_rx)),
        };

        let connection = Arc::new(connection);

        (connection.clone(), send_tx, connection.recv_queue.clone())
    }

    pub async fn start_connection(&self, recv_rx: Arc<Mutex<mpsc::Receiver<Vec<u8>>>>) {
        let arc_id = Arc::new(self.id.clone());
        let socket_clone_for_sender: Arc<Mutex<TcpStream>> = self.socket.clone();
        tokio::spawn(async move {
            let conn = CONNECTION_MANAGER
                .connections
                .get_mut(&*arc_id)
                .ok_or_else(|| {
                    error!("Connection not found");
                    ()
                });

            if let Ok(_) = conn {
                Connection::start_sender(socket_clone_for_sender, recv_rx).await;
            }
        });

        if let Err(e) = Connection::start_receiver(self.socket.clone()).await {
            error!("Receiver task failed with error: {}", e);
        }
    }

    async fn start_sender(
        socket: Arc<Mutex<TcpStream>>,
        recv_rx: Arc<Mutex<mpsc::Receiver<Vec<u8>>>>,
    ) {
        loop {
            while let Some(packet) = recv_rx.lock().await.recv().await {
                socket.lock().await.write_all(&packet).await.unwrap();
            }

            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
    }

    async fn start_receiver(
        socket: Arc<Mutex<TcpStream>>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut socket = socket.lock().await;

        let mut length_buffer = [0u8; 1];
        socket.read_exact(&mut length_buffer).await?;
        let length = length_buffer[0] as usize;

        let mut buffer = vec![0u8; length];
        socket.read_exact(&mut buffer).await?;

        let mut cursor = Cursor::new([length_buffer.to_vec(), buffer].concat());

        let packet_length = PacketManager::read_var_int(&mut cursor).await?;
        let packet_id = PacketManager::read_var_int(&mut cursor).await?;

        match packet_id.to_i32() {
            0x00 => {
                let handshake_packet = HandshakePacket::decode(&mut cursor).await.unwrap();
                debug!("{}", handshake_packet);
            }
            _ => {
                warn!(
                    "Unknown packet id {} with length {}",
                    packet_id, packet_length
                );
            }
        }

        Ok(())
    }
}

#[derive(PartialEq, Debug)]
pub enum ConnectionState {
    Unknown,
    Handshake,
    Status,
    Login,
    Play,
}

pub struct ConnectionManager {
    pub connections: DashMap<u32, Connection>,
    pub connection_count: AtomicU32,
    pub send_queue: Vec<Vec<u8>>,
    pub state: ConnectionState,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: DashMap::new(),
            connection_count: AtomicU32::new(0),
            state: ConnectionState::Unknown,
            send_queue: Vec::new(),
        }
    }

    pub fn generate_id(&self) -> u32 {
        loop {
            let id = random::<u32>();
            if !self.connections.contains_key(&id) {
                return id;
            }
        }
    }

    pub fn add_connection(&self, connection: Connection) {
        self.connection_count.fetch_add(1, Ordering::Relaxed);
        self.connections.insert(connection.id, connection);
    }

    pub fn remove_connection(&self, id: u32) {
        if self.connections.remove(&id).is_some() {
            self.connection_count.fetch_sub(1, Ordering::Relaxed);
        }
    }
}

pub async fn handle_connection(socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let id = CONNECTION_MANAGER.generate_id();
    let (connection, _send_queue, recv_queue) = Connection::new(socket, id, None);

    CONNECTION_MANAGER.add_connection((*connection).clone());

    let connection_ref = CONNECTION_MANAGER
        .connections
        .get_mut(&id)
        .ok_or("Connection not found")?;

    connection_ref.start_connection(recv_queue).await;

    info!("Established connection with the id {}", id);

    Ok(())
}
