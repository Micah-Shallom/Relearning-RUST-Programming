use std::io::{self, Read};

pub fn module_main() {
    let tx_bytes: &[u8; 10] = b"\x01\x00\x00\x00\xfd\x2c\x01...";
    let mut slice = &tx_bytes[..];

    let mut buffer = [0; 4];
    slice.read(&mut buffer).unwrap();

    let num_inputs = read_compact_size(&mut slice).unwrap();
    println!("Number of inputs: {}", num_inputs);
}

fn read_compact_size(input: &mut &[u8]) -> io::Result<u64> {
    if input.is_empty() {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "short read"));
    }
    let prefix = input[0];
    *input = &input[1..];

    match prefix {
        0..=0xfc => Ok(prefix as u64), 
        0xfd => {
            if input.len() < 2 { return short_read_err(); }
            let mut bytes = [0u8; 2];
            bytes.copy_from_slice(&input[..2]);
            *input = &input[2..];
            Ok(u16::from_le_bytes(bytes) as u64)
        }
        0xfe => {
            if input.len() < 4 { return short_read_err() }
            let mut bytes = [0u8; 4];
            bytes.copy_from_slice(&input[..4]);
            *input = &input[4..];
            Ok(u32::from_le_bytes(bytes) as u64)
        }
        0xff => {
            if input.len() < 8 { return short_read_err()}
            let mut bytes = [0u8; 8];
            bytes.copy_from_slice(&input[..8]);
            *input = &input[8..];
            Ok(u64::from_le_bytes(bytes))
        }
    }
}

fn short_read_err() -> io::Result<u64> {
    Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Short buffer for compactSize"))
}