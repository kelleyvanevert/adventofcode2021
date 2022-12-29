use std::fmt::Debug;

use parse::{hextobin, parse_packet};

mod parse;

pub type PacketTypeId = usize;
pub const LITERAL: PacketTypeId = 4;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Packet {
    version: usize,
    type_id: PacketTypeId,
    number: usize,
    children: Vec<Packet>,
}

impl Packet {
    /// This can surely be done better, I think it should be able to be FnMut and also not a reference. But, it's good enough for now :)
    fn fold<T: Debug, F>(&self, initial: T, f: &F) -> T
    where
        F: Fn(T, &Packet) -> T,
    {
        let mut value = f(initial, self);
        for child in &self.children {
            value = child.fold(value, f);
        }
        value
    }
}

pub fn solve(s: &str) -> usize {
    let packet = parse_packet(&hextobin(s.trim()));

    packet.fold(0, &|total, p| total + p.version)
}

pub fn bonus(_s: &str) -> usize {
    42
}

#[test]
fn test_solve() {
    assert_eq!(solve("8A004A801A8002F478"), 16);
    assert_eq!(solve("620080001611562C8802118E34"), 12);
    assert_eq!(solve("C0015000016115A2E0802F182340"), 23);
    assert_eq!(solve("A0016C880162017C3686B18A3D4780"), 31);
}
