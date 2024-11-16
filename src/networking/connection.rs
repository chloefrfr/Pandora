use crate::{
    config,
    packet::manager::PacketManager,
    packets::pong::handle_pong,
    utils::{read_file, send_status_response::send_status_response},
};
use bytes::BytesMut;

use log::{error, info};
use std::{path::Path, sync::Arc};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::Mutex,
    time,
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
                                let _protocol_version = packet.read_var_int();
                                let _server_address = packet.read_string();
                                let _server_port = packet.read_unsigned_short();
                                let next_state = packet.read_var_int();

                                state = next_state;
                                info!("State updated to: {}", state);

                                if state == 1 {
                                    send_status_response(&socket, config.max_players).await;
                                }
                            }
                            1 => {
                                send_status_response(&socket, config.max_players).await;
                            }
                            2 => {
                                let mut success = PacketManager::new(BytesMut::new(), 0);
                                success.write_uuid("550e8400-e29b-41d4-a716-446655440000");
                                success.write_string(&packet.read_string());
                                socket
                                    .lock()
                                    .await
                                    .write_all(&success.build_packet(0x02))
                                    .await
                                    .unwrap();

                                let mut join_game = PacketManager::new(BytesMut::new(), 0);
                                join_game.write_int(0);
                                join_game.write_boolean(true);
                                join_game.write_unsigned_byte(1);
                                join_game.write_byte(-1);
                                join_game.write_var_int(3);
                                join_game.write_string("minecraft:overworld");

                                let val =
                                    read_file::read_file("./NBT/dimension_codec.nbt").unwrap();

                                println!("{:?}", val);

                                join_game.append_blob(&val);

                                join_game.write_string("minecraft:overworld");
                                join_game.write_long(1);
                                join_game.write_long(10);
                                join_game.write_long(10);
                                join_game.write_boolean(false);
                                join_game.write_boolean(true);
                                join_game.write_boolean(false);
                                join_game.write_boolean(true);

                                socket
                                    .lock()
                                    .await
                                    .write_all(&join_game.build_packet(0x24))
                                    .await
                                    .unwrap();

                                let chunk_path = Path::new("./NBT/chunk.nbt");

                                for x in -8..8 {
                                    for z in -8..8 {
                                        let mut file = File::open(chunk_path).await.unwrap();
                                        let mut chunk_data = Vec::new();
                                        file.read_to_end(&mut chunk_data).await.unwrap();

                                        let mut chunk =
                                            PacketManager::new(BytesMut::from(&chunk_data[..]), 0);
                                        let mut chunk_data_packet =
                                            PacketManager::new(BytesMut::new(), 0);

                                        chunk_data_packet
                                            .append(&BytesMut::from(&(x as i32).to_be_bytes()[..]));
                                        chunk_data_packet
                                            .append(&BytesMut::from(&(z as i32).to_be_bytes()[..]));
                                        chunk.read_var_int();
                                        chunk.read_var_int();
                                        chunk.read_int();
                                        chunk.read_int();

                                        chunk_data_packet.extend_from_slice(
                                            &chunk.get_buffer()[chunk.get_offset()..],
                                        );

                                        socket
                                            .lock()
                                            .await
                                            .write_all(&chunk_data_packet.build_packet(0x20))
                                            .await
                                            .unwrap();
                                    }
                                }

                                let mut player_pos_and_look =
                                    PacketManager::new(BytesMut::new(), 0);
                                player_pos_and_look.write_double(0.0);
                                player_pos_and_look.write_double(64.0);
                                player_pos_and_look.write_double(0.0);
                                player_pos_and_look.write_float(0.0);
                                player_pos_and_look.write_float(0.0);
                                player_pos_and_look.write_byte(0);
                                player_pos_and_look.write_var_int(0);
                                socket
                                    .lock()
                                    .await
                                    .write_all(&player_pos_and_look.build_packet(0x34))
                                    .await
                                    .unwrap();

                                let mut interval = time::interval(time::Duration::from_secs(10));
                                loop {
                                    interval.tick().await;
                                    let mut keep_alive = PacketManager::new(BytesMut::new(), 0);
                                    keep_alive.write_long(0);
                                    socket
                                        .lock()
                                        .await
                                        .write_all(&keep_alive.build_packet(0x1F))
                                        .await
                                        .unwrap();
                                }
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
