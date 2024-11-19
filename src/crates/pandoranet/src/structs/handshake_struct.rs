use std::fmt::Display;

use packet_manager::types::varint_types::VarInt;
use pandora_macros::Decode;
use pandora_utils::types::decode_types::Decode;
use tokio::io::AsyncRead;
use tokio::io::AsyncSeek;

#[derive(Decode, Debug)]
pub struct HandshakePacket {
    protocol_version: VarInt,
    server_address: String,
    server_port: u16,
    next_state: VarInt,
}

impl Display for HandshakePacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HandshakePacket {{ protocol_version: {}, server_address: {}, server_port: {}, next_state: {} }}",
            self.protocol_version, self.server_address, self.server_port, self.next_state
        )
    }
}
