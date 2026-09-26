use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, ErrorKind, Read},
    path::Path,
};

use crate::models::Model;

pub fn load_glb<P: AsRef<Path>>(
    path: P,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    layout: &wgpu::BindGroupLayout,
) -> Model {
    const MAGIC: u32 = 0x46546c67;

    let path = path.as_ref();
    let base_dir = path.parent().unwrap_or_else(|| Path::new("."));

    let file = File::open(path).expect("Failed to open OBJ file");
    let mut reader = BufReader::new(file);

    let magic: [u8; 4] = read(&mut reader, 4).try_into().unwrap();
    if u32::from_le_bytes(magic) != MAGIC {
        panic!("Given file is not a GLB file");
    }
    let _version = read(&mut reader, 4);
    let length = read(&mut reader, 4);

    Model {
        meshes: vec![],
        materials: HashMap::new(),
    }
}

fn read(reader: &mut impl Read, bytes: usize) -> Vec<u8> {
    let mut buffer = vec![0u8; bytes];
    reader
        .read_exact(&mut buffer)
        .expect("Failed to read from file");
    buffer
}

fn parse_chunk(mut reader: BufReader<File>) {
    const JSON_CHUNK: u32 = 0x4e4f534a;
    const BIN_CHUNK: u32 = 0x004e4942;

    loop {
        let mut length_bytes = [0u8; 4];
        match reader.read_exact(&mut length_bytes) {
            Ok(_) => {}
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                break;
            }
            Err(e) => panic!("Unexpected error"),
        }

        let chunk_length = u32::from_le_bytes(length_bytes) as usize;
        let chunk_type = u32::from_le_bytes(read(&mut reader, 4).try_into().unwrap());
        let chunk_data = read(&mut reader, chunk_length);

        match chunk_type {
            JSON_CHUNK => {}
            BIN_CHUNK => {}
            _ => {}
        }
    }
}
