use std::{panic, str::Chars};

pub fn part1(input: &str) -> u64 {
    Packet::parse(&mut HexDecoder::new(input)).get_version_sum()
}

pub fn part2(input: &str) -> u64 {
    Packet::parse(&mut HexDecoder::new(input)).evaluate()
}

struct HexDecoder<'a> {
    hex_data: Chars<'a>,
    buffer: u32,
    buffer_len: usize,
    total_bits_read: usize,
}

impl<'a> HexDecoder<'a> {
    fn new(hex_string: &'a str) -> Self {
        Self {
            hex_data: hex_string.chars(),
            buffer: 0,
            buffer_len: 0,
            total_bits_read: 0,
        }
    }

    fn get_bits(&mut self, n: usize) -> u32 {
        if n > 28 {
            panic!("cannot get more than 28 bits at once!");
        }
        while n > self.buffer_len {
            self.buffer_len += 4;
            self.buffer = (self.buffer << 4) | self.hex_data.next().unwrap().to_digit(16).unwrap();
        }
        self.buffer_len -= n;
        self.total_bits_read += n;
        (self.buffer >> self.buffer_len) & ((1 << n) - 1)
    }

    fn get_total_bits_read(&self) -> usize {
        self.total_bits_read
    }
}

enum PacketContent {
    Literal(u64),
    Operator(u32, Vec<Packet>),
}

impl PacketContent {
    fn parse(dec: &mut HexDecoder, type_id: u32) -> Self {
        if type_id == 4 {
            let mut val: u64 = 0;
            let mut stop = 1;
            while stop == 1 {
                stop = dec.get_bits(1);
                val = (val << 4) | (dec.get_bits(4) as u64);
            }
            Self::Literal(val)
        } else {
            let mut sub_packets = Vec::new();
            if dec.get_bits(1) == 0 {
                let stop_at = (dec.get_bits(15) as usize) + dec.get_total_bits_read();
                while dec.get_total_bits_read() < stop_at {
                    sub_packets.push(Packet::parse(dec));
                }
            } else {
                for _ in 0..dec.get_bits(11) {
                    sub_packets.push(Packet::parse(dec));
                }
            }
            Self::Operator(type_id, sub_packets)
        }
    }

    fn evaluate(self) -> u64 {
        match self {
            Self::Literal(val) => val,
            Self::Operator(type_id, sub_packets) => {
                let mut sub_packet_values = sub_packets.into_iter().map(Packet::evaluate);
                let mut get_next_packet_value = || sub_packet_values.next().unwrap();
                match type_id {
                    0 => sub_packet_values.sum(),
                    1 => sub_packet_values.product(),
                    2 => sub_packet_values.min().unwrap(),
                    3 => sub_packet_values.max().unwrap(),
                    5 => (get_next_packet_value() > get_next_packet_value()) as u64,
                    6 => (get_next_packet_value() < get_next_packet_value()) as u64,
                    7 => (get_next_packet_value() == get_next_packet_value()) as u64,
                    _ => panic!("unknown packet type_id : {}", type_id),
                }
            }
        }
    }
}

struct Packet {
    version: u32,
    content: PacketContent,
}

impl Packet {
    fn parse(dec: &mut HexDecoder) -> Self {
        let version = dec.get_bits(3);
        let type_id = dec.get_bits(3);
        let content = PacketContent::parse(dec, type_id);
        Self { version, content }
    }

    fn get_version_sum(&self) -> u64 {
        let mut version_sum = self.version.into();
        if let PacketContent::Operator(_, sub_packets) = &self.content {
            version_sum += sub_packets.iter().map(|p| p.get_version_sum()).sum::<u64>();
        }
        version_sum
    }

    fn evaluate(self) -> u64 {
        self.content.evaluate()
    }
}
