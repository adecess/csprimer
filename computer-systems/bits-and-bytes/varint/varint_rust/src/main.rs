use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("150.uint64")?;
    let mut buffer = [0u8; 8];

    file.read_exact(&mut buffer)?;
    let num = u64::from_be_bytes(buffer);

    let encoding = encode(num);
    decode(encoding);

    Ok(())
}

fn encode(mut int_to_encode: u64) -> Vec<u8> {
    let mut encoding: Vec<u8> = vec![];

    while int_to_encode > 0 {
        let mut byte = (int_to_encode % 2u64.pow(7)) as u8;
        int_to_encode >>= 7;
        if int_to_encode > 0 {
            byte |= 2u8.pow(7);
        }
        encoding.push(byte);
    }

    for (index, elem) in encoding.iter().enumerate() {
        println!("byte sequence element {} {:#x}", index, elem);
    }
    encoding
}

fn decode(byte_sequence: Vec<u8>) -> u64 {
    let decoded_int = byte_sequence.iter().rev().fold(0u64, |acc, byte| {
        let shifted_acc = acc << 7;
        let bits = (byte & 0b01111111) as u64;
        shifted_acc | bits
    });

    println!("the decoded integer is: {}", decoded_int);
    decoded_int
}
