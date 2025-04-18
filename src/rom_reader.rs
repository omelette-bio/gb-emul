use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub fn read_n_bytes_at_offset(mut file : &File, offset: u64, bytes_readen: usize) -> std::io::Result<Vec<u8>>
{
    file.seek(SeekFrom::Start(offset))?;
    let mut buf = vec![0u8;bytes_readen];
    file.read_exact(&mut buf)?;

    Ok(buf)
}

pub fn read_byte_at_offset(mut file: &File, offset: u64) -> std::io::Result<u8>
{
    Ok(read_n_bytes_at_offset(file, offset, 1)?[0])
}
