#![feature(box_into_inner)]

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
    pub send_queue_sender: mpsc::Sender<Vec<u8>>,
    pub send_queue_receiver: Arc<Mutex<mpsc::Receiver<Vec<u8>>>>,
    pub state: ConnectionState,
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
        let (send_queue_sender, send_queue_receiver) = mpsc::channel::<Vec<u8>>(100);

        let connection = Connection {
            id,
            socket: Arc::new(Mutex::new(socket)),
            player_uuid,
            send_queue_sender,
            send_queue_receiver: Arc::new(Mutex::new(send_queue_receiver)),
            state: ConnectionState::Unknown,
        };

        let connection = Arc::new(connection);

        (
            connection.clone(),
            connection.send_queue_sender.clone(),
            connection.send_queue_receiver.clone(),
        )
    }

    pub async fn start_connection(&mut self) {
        self.state = ConnectionState::Handshake;
        let arc_id = Arc::new(self.id);

        tokio::spawn(async move {
            let conn = CONNECTION_MANAGER.connections.get_mut(&*arc_id);

            let Some(mut conn) = conn else {
                error!("Connection not found for id: {}", *arc_id);
                return;
            };

            let res = conn.start_sender().await;

            if let Err(e) = res {
                error!("Error in sender: {:?}", e);
            }
        });

        if let Err(e) = Connection::start_receiver(self, self.socket.clone()).await {
            error!("Receiver task failed with error: {}", e);
        }
    }

    async fn start_sender(&mut self) -> Result<(), std::io::Error> {
        loop {
            match self.send_queue_receiver.lock().await.recv().await {
                Some(packet) => {
                    if let Err(e) = self.socket.lock().await.write_all(&packet).await {
                        error!("Failed to write to socket: {:?}", e);
                        break;
                    }
                }
                None => {}
            }
        }
        Ok(())
    }

    async fn start_receiver(
        &mut self,
        socket: Arc<Mutex<TcpStream>>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut socket = socket.lock().await;

        loop {
            let mut length_buffer = [0u8; 1];
            if socket.read_exact(&mut length_buffer).await.is_err() {
                error!("Failed to read packet length.");
                break;
            }

            let length = length_buffer[0] as usize;
            let mut buffer = vec![0u8; length];

            if socket.read_exact(&mut buffer).await.is_err() {
                error!("Failed to read packet data.");
                break;
            }

            let mut cursor = Cursor::new([length_buffer.to_vec(), buffer].concat());

            let packet_length = PacketManager::read_var_int(&mut cursor).await?;
            let packet_id = PacketManager::read_var_int(&mut cursor).await?;

            match packet_id.to_i32() {
                0x00 => {
                    let handshake_packet = HandshakePacket::decode(&mut cursor).await.unwrap();
                    debug!("{}", handshake_packet);
                    handshake_packet.handle(self).await?;
                }
                _ => {
                    warn!(
                        "Unknown packet id {} with length {}",
                        packet_id, packet_length
                    );
                }
            }
        }
        Ok(())
    }

    pub async fn push_to_queue(&self, packet: Vec<u8>) {
        if let Err(e) = self.send_queue_sender.send(packet).await {
            error!("Failed to send packet to queue: {:?}", e);
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
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
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: DashMap::new(),
            connection_count: AtomicU32::new(0),
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
    let (connection, _send_queue_sender, _send_queue_receiver) = Connection::new(socket, id, None);

    CONNECTION_MANAGER.add_connection((*connection).clone());

    let mut connection_ref = CONNECTION_MANAGER
        .connections
        .get_mut(&id)
        .ok_or("Connection not found")?;

    connection_ref.start_connection().await;
    info!("Established connection with the id {}", id);

    Ok(())
}
