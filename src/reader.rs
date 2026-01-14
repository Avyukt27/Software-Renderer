use std::{
    fs::File,
    io::{self, Read},
};

fn read_bytes(file: &mut File, size: usize) -> io::Result<Vec<u8>> {
    let mut buf = vec![0u8; size];
    file.read_exact(&mut buf)?;
    Ok(buf)
}

pub fn read_header
