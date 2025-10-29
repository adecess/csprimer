use std::fs::File;
use std::io::{Error, Read};

fn main() -> std::io::Result<()> {
    let mut f = File::open("teapot.bmp")?;

    let mut buffer = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer)?;

    // find offset (starting address of pixel array) at the end of the file header
    // last 4 bytes of the header
    let offset_array: [u8; 4] = buffer[10..14]
        .try_into()
        .map_err(|_| Error::other("Could not convert header slice into array"))?;
    let offset = u32::from_le_bytes(offset_array) as usize;

    println!("The offset is {:#x}", offset);

    // first pixel
    println!("The first pixel is {:#x?}", &buffer[offset..(offset + 3)]);

    Ok(())
}
