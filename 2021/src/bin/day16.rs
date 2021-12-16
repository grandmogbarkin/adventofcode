use std::error::Error;

use advent2021::bitstream::BitStream;
use advent2021::exercise::{Exercise, SolutionT};
use advent2021::read;

struct Solution {}

impl SolutionT for Solution {
    fn day(&self) -> &str {
        "16"
    }

    fn test1_result(&self) -> i64 {
        31
    }
    fn test2_result(&self) -> i64 {
        2
    }

    fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        let input = read::read_lines(filename)?;
        Ok(run_task1(&input[0]))
    }

    fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        let input = read::read_lines(filename)?;
        Ok(run_task2(&input[0]))
    }
}

fn packet_from_hex(input_hex: &str) -> Packet {
    let input = read::parse_hex(input_hex);
    let mut stream = BitStream::new(&input);
    Packet::new(&mut stream)
}

fn run_task1(input_hex: &str) -> i64 {
    let packet = packet_from_hex(input_hex);
    // println!("{:?}", packet);
    packet.packet_version_sum()
}

fn run_task2(input_hex: &str) -> i64 {
    let packet = packet_from_hex(input_hex);
    packet.execute()
}

#[derive(Default, Debug, PartialEq)]
struct Packet {
    version: u8,
    type_id: u8,
    literal: i64,
    size: usize,
    sub_packets: Vec<Packet>,
}

impl Packet {
    pub fn new(stream: &mut BitStream) -> Self {
        let mut packet: Packet = Default::default();
        packet.parse(stream);
        packet
    }

    pub fn execute(&self) -> i64 {
        match self.type_id {
            0 => {
                //sum
                self.sub_packets.iter().fold(0, |sum, p| sum + p.execute())
            }
            1 => {
                //product
                if self.sub_packets.is_empty() {
                    return 0;
                }
                self.sub_packets
                    .iter()
                    .fold(1, |prod, p| prod * p.execute())
            }
            2 => {
                //min
                self.sub_packets.iter().fold(i64::MAX, |i, p| {
                    let sub = p.execute();
                    if sub < i {
                        sub
                    } else {
                        i
                    }
                })
            }
            3 => {
                //max
                self.sub_packets.iter().fold(0, |i, p| {
                    let sub = p.execute();
                    if sub > i {
                        sub
                    } else {
                        i
                    }
                })
            }
            4 => self.literal,
            5 => {
                //>
                if self.sub_packets[0].execute() > self.sub_packets[1].execute() {
                    1
                } else {
                    0
                }
            }
            6 => {
                //<
                if self.sub_packets[0].execute() < self.sub_packets[1].execute() {
                    1
                } else {
                    0
                }
            }
            7 => {
                //==
                if self.sub_packets[0].execute() == self.sub_packets[1].execute() {
                    1
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    pub fn parse(&mut self, stream: &mut BitStream) {
        self.version = self.read_bits(stream, 3) as u8;
        self.type_id = self.read_bits(stream, 3) as u8;
        // println!("Parsed: {}", self.type_id);
        match self.type_id {
            4 => {
                self.literal = self.parse_literal(stream);
            }
            _ => {
                self.parse_operator(stream);
            }
        }
    }

    fn parse_literal(&mut self, stream: &mut BitStream) -> i64 {
        let mut res: i64 = 0;
        let mut bit = 1;
        while bit > 0 {
            let group = self.read_bits(stream, 5);
            res <<= 4;
            res += (group & 0xF) as i64;
            bit = group >> 4;
        }
        // println!("Found literal: {}", res);
        res
    }

    fn parse_operator(&mut self, stream: &mut BitStream) {
        let length_type = self.read_bits(stream, 1) as u8;
        match length_type {
            0 => {
                let sub_packet_size = self.read_bits(stream, 15) as usize;
                // println!("Found operator {} with size {}", self.type_id, sub_packet_size);
                let mut sub_packet_size_sum: usize = 0;
                while sub_packet_size_sum < sub_packet_size {
                    let sub_packet = Packet::new(stream);
                    sub_packet_size_sum += sub_packet.packet_size();
                    self.sub_packets.push(sub_packet);
                }
            }
            1 => {
                let no_of_sub_packets = self.read_bits(stream, 11) as usize;
                // println!("Found operator {} with {} packets", self.type_id, no_of_sub_packets);
                for _ in 0..no_of_sub_packets {
                    let sub_packet = Packet::new(stream);
                    self.sub_packets.push(sub_packet);
                }
            }
            _ => (),
        }
    }

    fn packet_size(&self) -> usize {
        let mut size = self.size;
        for p in &self.sub_packets {
            size += p.packet_size();
        }
        size
    }

    fn packet_version_sum(&self) -> i64 {
        let mut ver_sum: i64 = self.version as i64;
        for p in &self.sub_packets {
            ver_sum += p.packet_version_sum();
        }
        ver_sum
    }

    fn read_bits(&mut self, stream: &mut BitStream, len: usize) -> u16 {
        self.size += len;
        stream.read_bits(len)
    }
}

pub fn main() {
    let d = Solution {};
    Exercise::run(&d, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_1_is_working() {
        assert_eq!(16, run_task1("8A004A801A8002F478"));
        assert_eq!(12, run_task1("620080001611562C8802118E34"));
        assert_eq!(23, run_task1("C0015000016115A2E0802F182340"));
        assert_eq!(31, run_task1("A0016C880162017C3686B18A3D4780"));
    }

    #[test]
    fn task_2_is_working() {
        // 1 + 2
        assert_eq!(3, run_task2("C200B40A82"));
        // 6 * 9
        assert_eq!(54, run_task2("04005AC33890"));
        // Min(7,8,9)
        assert_eq!(7, run_task2("880086C3E88112"));
        // Max(7,8,9)
        assert_eq!(9, run_task2("CE00C43D881120"));
        // 5 < 15
        assert_eq!(1, run_task2("D8005AC2A8F0"));
        // 5 > 15
        assert_eq!(0, run_task2("F600BC2D8F"));
        // 5 == 15
        assert_eq!(0, run_task2("9C005AC2F8F0"));
        // 1 + 3 == 2 * 2
        assert_eq!(1, run_task2("9C0141080250320F1802104A08"));
    }
}
