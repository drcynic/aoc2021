use bitstream_io::{BigEndian, BitRead, BitReader};
use std::fs;

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap().trim().to_string();
    let bytes = hex::decode(&input).unwrap();
    let mut r = BitReader::endian(bytes.as_slice(), BigEndian);
    let (version, _) = read_packet(&mut r);

    println!("Version sum: {}", version);
}

fn read_packet(r: &mut BitReader<&[u8], BigEndian>) -> (u64, u64) {
    let version = r.read_var::<u64>(3).unwrap();
    let id = r.read_var::<u8>(3).unwrap();

    let (v, bits_read) = match id {
        4 => read_literal(r),
        _ => read_operator(r),
    };

    (version + v, bits_read + 6)
}

fn read_literal(r: &mut BitReader<&[u8], BigEndian>) -> (u64, u64) {
    let mut values = Vec::new();
    let mut bits_read = 0;

    loop {
        let g = r.read_bit().unwrap();
        let a = r.read_var::<u64>(4).unwrap();
        values.push(a);
        bits_read += 5;
        if !g {
            break;
        }
    }
    let _v = values.iter().rev().enumerate().fold(0, |acc, (i, &value)| acc + (value << (i * 4)));

    (0, bits_read)
}

fn read_operator(r: &mut BitReader<&[u8], BigEndian>) -> (u64, u64) {
    let mut bits_read: u64 = 1;
    let mut version = 0;
    if r.read_bit().unwrap() {
        let num_packet_bits = 11;
        bits_read += 11;
        let num_packets = r.read_var::<u64>(num_packet_bits).unwrap();
        for _ in 0..num_packets {
            let (v, bits) = read_packet(r);
            bits_read += bits;
            version += v;
        }
    } else {
        let length_bits = 15;
        bits_read += 15;
        let length = r.read_var::<u64>(length_bits).unwrap();
        let mut read_bits_length: u64 = 0;
        while read_bits_length < length {
            let (v, bits_read) = read_packet(r);
            read_bits_length += bits_read;
            version += v;
        }
        bits_read += length;
    }

    (version, bits_read)
}
