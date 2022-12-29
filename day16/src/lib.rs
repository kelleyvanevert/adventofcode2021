use std::fmt::Debug;

use parse::{hextobin, parse_packet};

mod parse;

pub type PacketTypeId = usize;
pub const LITERAL: PacketTypeId = 4;
pub const OP_SUM: PacketTypeId = 0;
pub const OP_PROD: PacketTypeId = 1;
pub const OP_MIN: PacketTypeId = 2;
pub const OP_MAX: PacketTypeId = 3;
pub const OP_GT: PacketTypeId = 5;
pub const OP_LT: PacketTypeId = 6;
pub const OP_EQ: PacketTypeId = 7;

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

    fn eval(&self) -> usize {
        match self.type_id {
            LITERAL => self.number,
            OP_SUM => self.children.iter().map(|p| p.eval()).sum(),
            OP_PROD => self.children.iter().map(|p| p.eval()).product(),
            OP_MIN => self.children.iter().map(|p| p.eval()).min().unwrap(),
            OP_MAX => self.children.iter().map(|p| p.eval()).max().unwrap(),
            OP_GT => (self.children[0].eval() > self.children[1].eval()).into(),
            OP_LT => (self.children[0].eval() < self.children[1].eval()).into(),
            OP_EQ => (self.children[0].eval() == self.children[1].eval()).into(),
            _ => unreachable!(),
        }
    }
}

pub fn solve(s: &str) -> usize {
    let packet = parse_packet(&hextobin(s.trim()));

    packet.fold(0, &|total, p| total + p.version)
}

pub fn bonus(s: &str) -> usize {
    let packet = parse_packet(&hextobin(s.trim()));

    packet.eval()
}

#[test]
fn test_solve() {
    assert_eq!(solve("8A004A801A8002F478"), 16);
    assert_eq!(solve("620080001611562C8802118E34"), 12);
    assert_eq!(solve("C0015000016115A2E0802F182340"), 23);
    assert_eq!(solve("A0016C880162017C3686B18A3D4780"), 31);
}

#[test]
fn test_bonus() {
    assert_eq!(bonus("C200B40A82"), 3);
    assert_eq!(bonus("04005AC33890"), 54);
    assert_eq!(bonus("880086C3E88112"), 7);
    assert_eq!(bonus("CE00C43D881120"), 9);
    assert_eq!(bonus("D8005AC2A8F0"), 1);
    assert_eq!(bonus("F600BC2D8F"), 0);
    assert_eq!(bonus("9C005AC2F8F0"), 0);
    assert_eq!(bonus("9C0141080250320F1802104A08"), 1);
}
