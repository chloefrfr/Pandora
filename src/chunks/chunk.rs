#[derive(Clone)]
pub struct Chunk {
    pub x: i32,
    pub z: i32,
    pub data: Vec<u8>,
}

impl Chunk {
    pub fn new(x: i32, z: i32, data: Vec<u8>) -> Self {
        Chunk { x, z, data }
    }
}
