use std::io::{Read, Result};

#[derive(Debug)]
pub struct GlbHeader {
    magic: u32,
    version: u32,
    length: u32,
}

impl GlbHeader {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
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
