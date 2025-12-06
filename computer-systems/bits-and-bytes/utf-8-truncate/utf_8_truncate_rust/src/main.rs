use std::fs::File;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let mut f = File::open("cases")?;
    let mut buffer: Vec<u8> = vec![];

    f.read_to_end(&mut buffer)?;

    let mut lines: Vec<Vec<u8>> = buffer
        .split(|b| *b == b'\n')
        .map(|bytes| bytes.to_vec())
        .filter(|line| line.len() > 0)
        .collect();

    println!("Lines are {:#?}", lines);

    let mut new_file = File::create("new-cases")?;

    for line in lines.iter_mut() {
        // truncate
        let mut i = line[0] as usize;
        i += 1;

        while i < line.len() {
            if get_byte_position(line[i]) == Utf8BytePattern::ContinuationByte {
                // Move back one index (but don't go negative)
                if i > 0 {
                    i -= 1;
                }
            } else {
                line.truncate(i);
            }
        }

        // remove first byte
        if line.len() > 0 {
            line.remove(0);
        }
        // add line break
        line.push(b'\n');
    }

    let truncated_lines: Vec<u8> = lines.into_iter().flatten().collect();

    new_file.write_all(&truncated_lines)?;
    println!("Created new rotated file");

    Ok(())
}

#[derive(PartialEq)]
enum Utf8BytePattern {
    ContinuationByte,
    FirstByteOfOne,
    FirstByteOfTwo,
    FirstByteOfThree,
    FirstByteOfFour,
}

fn get_byte_position(b: u8) -> Utf8BytePattern {
    match b {
        b if b & 0x80 == 0 => Utf8BytePattern::FirstByteOfOne,
        b if b & 0xE0 == 0xC0 => Utf8BytePattern::FirstByteOfTwo,
        b if b & 0xF0 == 0xE0 => Utf8BytePattern::FirstByteOfThree,
        b if b & 0xF8 == 0xF0 => Utf8BytePattern::FirstByteOfFour,
        _ => Utf8BytePattern::ContinuationByte,
    }
}
