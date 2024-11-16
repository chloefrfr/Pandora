use crate::{
    config, packet::manager::PacketManager, packets::pong::handle_pong,
    utils::send_status_response::send_status_response,
};
use bytes::BytesMut;
use log::error;
use std::sync::Arc;
use tokio::{
    io::AsyncReadExt,
    net::TcpStream,
    sync::Mutex,
};

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
    let mut proto_version = 0;
    let mut buffer = BytesMut::new();

    loop {
        let mut data = vec![0; 1024];
        let read_result = {
            let mut socket_guard = socket.lock().await;
            socket_guard.read(&mut data).await
        };

        match read_result {
            Ok(0) => break, // Connection closed
            Ok(n) => {
                buffer.extend_from_slice(&data[..n]);
                let mut packet = PacketManager::new(buffer.clone(), 0);

                while packet.get_buffer().len() > 0 {
                    let packet_id = packet.read_var_int();
                    match packet_id as i32 {
                        0x00 => match state {
                            0 => {
                                let protocol_version = packet.read_var_int();
                                let _server_address = packet.read_string();
                                let _server_port = packet.read_unsigned_short();
                                let next_state = packet.read_var_int();

                                state = next_state;
                                proto_version = protocol_version;

                                if state == 1 {
                                    send_status_response(
                                        &socket,
                                        proto_version,
                                        config.max_players,
                                    )
                                    .await;
                                }
                            }
                            1 => {
                                send_status_response(&socket, proto_version, config.max_players)
                                    .await;
                            }
                            _ => error!("Unrecognized state: {}", state),
                        },
                        0x01 if state == 1 => {
                            let mut socket_guard = socket.lock().await;
                            handle_pong(&mut buffer, &mut socket_guard).await;
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
