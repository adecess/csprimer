use std::fs::File;
use std::io::{Error, Read};

fn main() -> std::io::Result<()> {
    let mut f = File::open("synflood.pcap")?;
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer)?;

    let magic_number = get_four_bytes(&buffer[0..4])?;
    let major_version = get_two_bytes(&buffer[4..6])?;
    let minor_version = get_two_bytes(&buffer[6..8])?;
    let _link_layer_header = get_four_bytes(&buffer[20..24])?;

    assert_eq!(magic_number, 0xa1b2c3d4);
    println!("Version number is {}.{}", major_version, minor_version);

    Ok(())
}

fn get_four_bytes(buffer_slice: &[u8]) -> std::io::Result<usize> {
    let array: [u8; 4] = buffer_slice
        .try_into()
        .map_err(|_| Error::other("Could not convert header slice into array"))?;

    Ok(u32::from_le_bytes(array) as usize)
}

fn get_two_bytes(buffer_slice: &[u8]) -> std::io::Result<usize> {
    let array: [u8; 2] = buffer_slice
        .try_into()
        .map_err(|_| Error::other("Could not convert header slice into array"))?;

    Ok(u16::from_le_bytes(array) as usize)
}
