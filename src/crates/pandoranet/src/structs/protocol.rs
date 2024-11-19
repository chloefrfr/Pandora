use packet_manager::types::varint_types::VarInt;
use pandora_macros::Encode;
use pandora_utils::types::encode_types::Encode;

#[derive(Encode, Debug)]
pub struct ClientHandshakeRequest {
    pub packet_id: VarInt,
    pub res_json: String,
}
