use std::io::{Read, Result};

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug)]
pub struct GlbHeader {
    pub magic: u32,
    pub version: u32,
    pub length: u32,
}

impl GlbHeader {
    pub fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        use byteorder::{LittleEndian, ReadBytesExt};

        let magic = reader.read_u32::<LittleEndian>()?;
        let version = reader.read_u32::<LittleEndian>()?;
        let length = reader.read_u32::<LittleEndian>()?;

        Ok(Self {
            magic,
            version,
            length,
        })
    }
}

#[derive(Debug)]
pub struct GlbChunk {
    pub chunk_length: u32,
    pub chunk_type: u32,
    pub chunk_data: Vec<u8>,
}

impl GlbChunk {
    pub fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let chunk_length = reader.read_u32::<LittleEndian>()?;
        let chunk_type = reader.read_u32::<LittleEndian>()?;

        let mut chunk_data = vec![0u8; chunk_length as usize];
        reader.read_exact(&mut chunk_data)?;

        Ok(Self {
            chunk_length,
            chunk_type,
            chunk_data,
        })
    }
}
