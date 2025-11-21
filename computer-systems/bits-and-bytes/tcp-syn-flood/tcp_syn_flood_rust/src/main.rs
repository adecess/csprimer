use std::fs::File;
use std::io::{Error, Read};

fn main() -> std::io::Result<()> {
    let mut f = File::open("synflood.pcap")?;
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer)?;

    let magic_number = get_four_bytes(&buffer[0..4])?;
    let major_version = get_two_bytes(&buffer[4..6])?;
    let minor_version = get_two_bytes(&buffer[6..8])?;
    let link_layer_header_type = get_four_bytes(&buffer[20..24])?;

    assert_eq!(magic_number, 0xa1b2c3d4);
    println!("Version number is {}.{}", major_version, minor_version);
    assert_eq!(link_layer_header_type, 0);

    let mut packet_data_list = Vec::new();
    let mut slice: &[u8] = &buffer[24..];
    let mut initiated = 0;
    let mut acked = 0;

    while !slice.is_empty() {
        let captured_packet_length = get_four_bytes(&slice[8..12])?;
        let untruncated_packet_length = get_four_bytes(&slice[12..16])?;
        assert_eq!(captured_packet_length, untruncated_packet_length);

        let data_start = 16;
        let link_layer_header = get_four_bytes(&slice[data_start..(data_start + 4)])?;
        assert_eq!(link_layer_header, 2); // payload is an ipv4 packet

        let ipv4_start = data_start + 4;
        // the internet header length is in the first byte of the ipv4 header
        let ihl = (slice[ipv4_start] as usize & 0x0f) * 4; // we mask the high order bits (big endian) + the ipv4 header is made of 4-byte words
        assert_eq!(ihl, 20);

        let tcp_start = ipv4_start + ihl;
        // tcp header where ipv4 header ends
        let source_port = get_two_bytes_be(&slice[tcp_start..(tcp_start + 2)])?;
        let destination_port = get_two_bytes_be(&slice[(tcp_start + 2)..(tcp_start + 4)])?;
        let syn_ack_byte = slice[tcp_start + 13];
        let syn_flag = syn_ack_byte & 0x02 > 0;
        let ack_flag = syn_ack_byte & 0x10 > 0;

        if destination_port == 80 && syn_flag {
            initiated += 1;
        } else if source_port == 80 && ack_flag {
            acked += 1;
        }

        println!(
            "from port {}, to port {}, {} and {}",
            source_port,
            destination_port,
            if syn_flag { "SYN" } else { "" },
            if ack_flag { "ACK" } else { "" }
        );

        let data_end = data_start + captured_packet_length;

        let packet_data = slice[data_start..data_end].to_vec();
        packet_data_list.push(packet_data);

        slice = &slice[data_end..];
    }

    println!(
        "There is a total of {} packets. {} are SYN packets, and {} are ACK packets hence a {:.2} ACK percentage",
        packet_data_list.len(),
        initiated,
        acked,
        acked as f32 / initiated as f32
    );

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

fn get_two_bytes_be(buffer_slice: &[u8]) -> std::io::Result<usize> {
    let array: [u8; 2] = buffer_slice
        .try_into()
        .map_err(|_| Error::other("Could not convert header slice into array"))?;

    Ok(u16::from_be_bytes(array) as usize)
}
