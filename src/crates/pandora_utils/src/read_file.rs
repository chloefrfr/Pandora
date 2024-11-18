use std::{
    fs::File,
    io::{Cursor, Read},
    path::Path,
};

use nbt::Blob;

pub fn read_file(path: &str) -> Result<Blob, Box<dyn std::error::Error>> {
    let path = Path::new(path);
    let mut file = File::open(path).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    let blob = Blob::from_reader(&mut Cursor::new(data)).unwrap();

    Ok(blob)
}
