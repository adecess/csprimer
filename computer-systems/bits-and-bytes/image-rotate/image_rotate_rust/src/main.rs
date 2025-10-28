use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut f = File::open("teapot.bmp")?;
    let mut buffer = [0u8; 2];

    // read 2 bytes
    f.read_exact(&mut buffer)?;

    println!("The bytes: {:#02x?}", &buffer);
    Ok(())
}
