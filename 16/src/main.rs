use bitstream_io::{BigEndian, BitRead, BitReader};
use num::FromPrimitive;
use std::fs;

#[macro_use]
extern crate num_derive;

#[derive(FromPrimitive)]
enum Op {
    Sum = 0,
    Product = 1,
    Min = 2,
    Max = 3,
    Gt = 5,
    Lt = 6,
    Eq = 7,
}

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap().trim().to_string();
    let bytes = hex::decode(&input).unwrap();
    let mut r = BitReader::endian(bytes.as_slice(), BigEndian);
    let (value, version, _) = read_packet(&mut r);

    println!("Part1 - version sum: {}", version);
    println!("Part2 - operations result: {}", value)
}

fn read_packet(r: &mut BitReader<&[u8], BigEndian>) -> (u64, u64, u64) {
    let version = r.read_var::<u64>(3).unwrap();
    let id = r.read_var::<u8>(3).unwrap();

    let (val, ver, bits_read) = match id {
        4 => read_literal(r),
        _ => read_operator(r, &FromPrimitive::from_u8(id).unwrap()),
    };

    (val, version + ver, bits_read + 6)
}

fn read_literal(r: &mut BitReader<&[u8], BigEndian>) -> (u64, u64, u64) {
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
    let v = values.iter().rev().enumerate().fold(0, |acc, (i, &value)| acc + (value << (i * 4)));

    (v, 0, bits_read)
}

fn read_operator(r: &mut BitReader<&[u8], BigEndian>, op: &Op) -> (u64, u64, u64) {
    let mut bits_read: u64 = 1;
    let mut version_sum = 0;
    let mut value = 0;
    let operation = |idx: u64, lhs: u64, rhs: u64, op: &Op| -> u64 {
        if idx == 0 {
            rhs
        } else {
            match op {
                Op::Sum => lhs + rhs,
                Op::Product => lhs * rhs,
                Op::Min => std::cmp::min(lhs, rhs),
                Op::Max => std::cmp::max(lhs, rhs),
                Op::Gt => (lhs > rhs) as u64,
                Op::Lt => (lhs < rhs) as u64,
                Op::Eq => (lhs == rhs) as u64,
            }
        }
    };

    if r.read_bit().unwrap() {
        let num_packet_bits = 11;
        bits_read += 11;
        let num_packets = r.read_var::<u64>(num_packet_bits).unwrap();
        for i in 0..num_packets {
            let (val, version, bits) = read_packet(r);
            bits_read += bits;
            version_sum += version;
            value = operation(i, value, val, &op);
        }
    } else {
        let length_bits = 15;
        bits_read += 15;
        let length = r.read_var::<u64>(length_bits).unwrap();
        let mut read_bits_length: u64 = 0;
        let mut i = 0;
        while read_bits_length < length {
            let (val, version, bits_read) = read_packet(r);
            read_bits_length += bits_read;
            version_sum += version;
            value = operation(i, value, val, &op);
            i += 1;
        }
        bits_read += length;
    }

    (value, version_sum, bits_read)
}
