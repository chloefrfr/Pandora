use crate::{
    config, constants,
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
        let mut connections_guard = connections.lock().await;
        connections_guard.push(socket.clone());
    }

    let mut state = 0;
    let mut buffer = BytesMut::new();

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
                        constants::HANDSHAKE_PACKET => match state {
                            0 => {
                                handshake::HandshakePacket
                                    .handle(&mut packet, socket.clone(), &mut state)
                                    .await
                                    .unwrap_or_else(|err| {
                                        error!("Failed to handle handshake packet: {}", err);
                                    });
                            }
                            1 => {
                                send_status_response(&socket, config.max_players).await;
                            }
                            2 => {
                                player_join::PlayerJoinPacket
                                    .handle(&mut packet, socket.clone(), &mut state)
                                    .await
                                    .unwrap_or_else(|err| {
                                        error!("Failed to handle player join packet: {}", err);
                                    });
                            }
                            _ => {}
                        },
                        constants::PONG_PACKET if state == 1 => {
                            let mut socket_guard = socket.lock().await;
                            handle_pong(&mut buffer, &mut socket_guard).await
                        }
                        constants::CHAT_MESSAGE_PACKET => {
                            let message = packet.read_string();
                            info!("Received chat message: {}", message);
                        }
                        _ => {}
                    }
                }
            }
            Err(err) => {
                error!("Error reading from socket: {}", err);
                break;
            }
        }
    }

    {
        let mut connections_guard = connections.lock().await;
        connections_guard.retain(|conn| !Arc::ptr_eq(conn, &socket));
    }
}
