use bytes::BytesMut;
use log::{debug, error, info};
use serde_json::json;
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

use crate::{config, packet::manager::PacketManager};

pub async fn handle_connection(
    mut socket: TcpStream,
    connections: Arc<Mutex<Vec<Arc<Mutex<TcpStream>>>>>,
) {
    let config = config::Config::load_config();
    let socket = Arc::new(Mutex::new(socket));

    {
        let mut conns = connections.lock().await;
        conns.push(socket.clone());
    }

    let mut state = 0;
    let mut proto_version = 0;
    let mut buffer = BytesMut::new();

    loop {
        let mut data = vec![0; 1024];

        let mut socket = socket.lock().await;

        match socket.read(&mut data).await {
            Ok(0) => break,
            Ok(n) => {
                buffer.extend_from_slice(&data[0..n]);
                let mut packet = PacketManager::new(buffer.clone(), 0);

                while packet.get_buffer().len() > 0 {
                    let packet_id = packet.read_var_int();

                    match packet_id as i32 {
                        0x00 => match state {
                            0 => {
                                let handshake = {
                                    let protocol_version = packet.read_var_int();
                                    let server_address = packet.read_string();
                                    let server_port = packet.read_unsigned_short();
                                    let next_state = packet.read_var_int();

                                    println!(
                                        "Handshake: {:?}",
                                        (protocol_version, server_address, server_port, next_state)
                                    );
                                    (next_state, protocol_version)
                                };
                                state = handshake.0;
                                proto_version = handshake.1;

                                if state == 1 {
                                    let mut response = PacketManager::new(BytesMut::new(), 0);
                                    response.write_string(
                                        &serde_json::to_string(&json!({
                                            "version": {
                                                "name": "1.21.3",
                                                "protocol": proto_version,
                                            },
                                            "players": {
                                                "max": config.max_players,
                                                "online": 0,
                                            },
                                        }))
                                        .unwrap(),
                                    );
                                    socket
                                        .write_all(&response.build_packet(0x00))
                                        .await
                                        .unwrap();
                                }
                            }
                            1 => {
                                let mut response = PacketManager::new(BytesMut::new(), 0);
                                response.write_string(
                                    &serde_json::to_string(&json!({
                                        "version": {
                                            "name": "1.21.3",
                                            "protocol": proto_version,
                                        },
                                        "players": {
                                            "max": config.max_players,
                                            "online": 0,
                                        },
                                    }))
                                    .unwrap(),
                                );
                                socket
                                    .write_all(&response.build_packet(0x00))
                                    .await
                                    .unwrap();
                            }
                            _ => {
                                println!("Unrecognized state: {}", state);
                            }
                        },
                        0x01 => {
                            if state == 1 {
                                let mut pong = PacketManager::new(BytesMut::new(), 0);
                                pong.write_long(packet.read_long());
                                socket.write_all(&pong.build_packet(0x01)).await.unwrap();
                            }
                        }
                        _ => {
                            error!("Unknown Packet ID: {}", packet_id as i32);
                        }
                    }
                }
            }
            Err(err) => eprintln!("Error reading from socket: {}", err),
        }
    }
}
