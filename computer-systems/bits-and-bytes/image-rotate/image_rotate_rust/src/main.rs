use std::fs::File;
use std::io::{Error, Read};

fn main() -> std::io::Result<()> {
    let mut f = File::open("teapot.bmp")?;

    let mut buffer = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer)?;

    // find offset as the last 4 bytes of the file header
    let offset_array: [u8; 4] = buffer[10..14]
        .try_into()
        .map_err(|_| Error::other("Could not convert header slice into array"))?;
    let offset = u32::from_le_bytes(offset_array) as usize;

    println!("The offset is {}", offset);

    // bitmap height and width
    let bitmap_width = i32::from_le_bytes(
        buffer[18..22]
            .try_into()
            .map_err(|_| Error::other("Could not convert bitmap width bytes into array"))?,
    );
    let bitmap_height = i32::from_le_bytes(
        buffer[22..26]
            .try_into()
            .map_err(|_| Error::other("Could not convert bitmap height bytes into array"))?,
    );

    println!(
        "File width is {} and height is {}",
        bitmap_width, bitmap_height
    );

    Ok(())
}
