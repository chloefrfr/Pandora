use dashmap::DashMap;
use lazy_static::lazy_static;
use rand::random;
use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
};
use tokio::{
    net::TcpStream,
    sync::{mpsc, Mutex},
};
use uuid::Uuid;

lazy_static! {
    pub static ref CONNECTION_MANAGER: ConnectionManager = ConnectionManager::new();
}

pub struct Connection {
    pub id: u32,
    pub socket: TcpStream,
    pub player_uuid: Option<Uuid>,
    pub send_queue: mpsc::Sender<Vec<u8>>,
    pub recv_queue: Arc<Mutex<mpsc::Receiver<Vec<u8>>>>,
}

impl Connection {
    pub fn new(
        socket: TcpStream,
        id: u32,
        player_uuid: Option<Uuid>,
    ) -> (Self, mpsc::Sender<Vec<u8>>) {
        let (send_tx, _send_rx) = mpsc::channel(1024);
        let (recv_tx, recv_rx) = mpsc::channel(1024);

        (
            Connection {
                id,
                socket,
                player_uuid,
                send_queue: recv_tx,
                recv_queue: Arc::new(Mutex::new(recv_rx)),
            },
            send_tx,
        )
    }

    pub async fn send(&self, data: Vec<u8>) {
        if let Err(err) = self.send_queue.send(data).await {
            eprintln!("Failed to send data to queue: {}", err);
        }
    }

    pub async fn receive(&self) -> Option<Vec<u8>> {
        let mut recv = self.recv_queue.lock().await;
        recv.recv().await
    }
}

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
    pub state: ConnectionState,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: DashMap::new(),
            connection_count: AtomicU32::new(0),
            state: ConnectionState::Unknown,
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
    let (connection, _send_queue) = Connection::new(socket, id, None);

    CONNECTION_MANAGER.add_connection(connection);

    Ok(())
}
