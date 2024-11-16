use crate::{
    config,
    packet::manager::{PacketHandler, PacketManager},
    packets::{handshake, player_join, pong::handle_pong},
    utils::send_status_response::send_status_response,
};
use bytes::BytesMut;

use log::{error, info};
use std::sync::Arc;
use tokio::{io::AsyncReadExt, net::TcpStream, sync::Mutex};

pub async fn handle_connection(
    socket: TcpStream,
    connections: Arc<Mutex<Vec<Arc<Mutex<TcpStream>>>>>,
) {
    let config = config::Config::load_config();
    let socket = Arc::new(Mutex::new(socket));
    {
        connections.lock().await.push(socket.clone());
    }

    let mut state = 0;
    let mut buffer: BytesMut = BytesMut::new();

    loop {
        let mut data = vec![0; 1024];
        let read_result = {
            let mut socket_guard = socket.lock().await;
            socket_guard.read(&mut data).await
        };

        match read_result {
            Ok(0) => {
                break;
            }
            Ok(n) => {
                buffer.extend_from_slice(&data[..n]);
                let mut packet = PacketManager::new(buffer.clone(), 0);

                while packet.get_buffer().len() > 0 {
                    let packet_id = packet.read_var_int();

                    match packet_id as i32 {
                        0x00 => match state {
                            0 => {
                                handshake::HandshakePacket
                                    .handle(&mut packet, socket.clone(), &mut state)
                                    .await
                                    .unwrap();
                            }
                            1 => {
                                send_status_response(&socket, config.max_players).await;
                            }
                            2 => {
                                player_join::PlayerJoinPacket
                                    .handle(&mut packet, socket.clone(), &mut state)
                                    .await
                                    .unwrap();
                            }
                            _ => error!("Unrecognized state: {}", state),
                        },
                        0x01 if state == 1 => {
                            let mut socket_guard = socket.lock().await;
                            handle_pong(&mut buffer, &mut socket_guard).await;
                        }
                        0x03 => {
                            let message = packet.read_string();
                            info!("Received chat message: {}", message);
                        }
                        _ => {
                            error!("Unknown Packet ID: {}", packet_id);
                        }
                    }
                }
            }
            Err(err) => {
                error!("Error reading from socket: {}", err);
                break;
            }
        }
    }
}
