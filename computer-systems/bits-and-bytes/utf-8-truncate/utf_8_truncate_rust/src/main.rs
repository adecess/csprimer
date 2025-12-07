use std::fs::File;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let mut f = File::open("cases")?;
    let mut buffer: Vec<u8> = vec![];

    f.read_to_end(&mut buffer)?;

    let mut lines: Vec<Vec<u8>> = buffer
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|bytes| bytes.to_vec())
        .collect();

    println!("Lines are {:#?}", lines);

    let mut new_file = File::create("new-cases")?;

    for line in &mut lines {
        if line.is_empty() {
            continue;
        }

        let mut i = line[0] as usize + 1;

        // Find the truncation point
        while i < line.len() && (line[i] & 0xC0 == 0x80) {
            // Move back one index (but don't go negative)
            i = i.saturating_sub(1);
        }

        line.truncate(i);

        // remove first byte
        line.remove(0);
        line.push(b'\n');

        // add line to file
        new_file.write_all(&line)?;
    }

    println!("Created new truncated file");

    Ok(())
}
