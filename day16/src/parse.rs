use crate::{Packet, LITERAL};
use nom::{
    branch::alt,
    character::complete::{char, one_of},
    combinator::{map, verify},
    error::Error,
    multi::{count, many0},
    sequence::{pair, preceded, tuple},
    IResult, Parser,
};

fn bitvec_to_num(v: Vec<char>) -> usize {
    let s = v.iter().collect::<String>();
    usize::from_str_radix(&s, 2).unwrap()
}

fn bits<'a>(n: usize) -> impl Parser<&'a str, Vec<char>, Error<&'a str>> {
    count(one_of("01"), n)
}

fn bits_value<'a>(n: usize) -> impl Parser<&'a str, usize, Error<&'a str>> {
    map(bits(n), bitvec_to_num)
}

fn bits_w_value<'a>(n: usize, value: usize) -> impl Parser<&'a str, usize, Error<&'a str>> {
    verify(bits_value(n), move |&v| v == value)
}

fn bits_neq_value<'a>(n: usize, value: usize) -> impl Parser<&'a str, usize, Error<&'a str>> {
    verify(bits_value(n), move |&v| v != value)
}

fn p_literal(s: &str) -> IResult<&str, Packet> {
    map(
        tuple((
            bits_value(3),
            bits_w_value(3, LITERAL),
            map(
                pair(
                    many0(preceded(char('1'), bits(4))),
                    preceded(char('0'), bits(4)),
                ),
                |(a, b)| {
                    bitvec_to_num(
                        a.into_iter()
                            .flatten()
                            .chain(b.into_iter())
                            .collect::<Vec<_>>(),
                    )
                },
            ),
        )),
        |(version, type_id, number)| Packet {
            version,
            type_id,
            number,
            children: vec![],
        },
    )(s)
}

fn packet_children<'a>(s: &'a str) -> IResult<&'a str, Vec<Packet>> {
    let (s, len_type_id) = one_of("01")(s)?;
    match len_type_id {
        '0' => children_upto_len(s),
        '1' => children_upto_amount(s),
        _ => unreachable!(),
    }
}

fn children_upto_len<'a>(s: &'a str) -> IResult<&'a str, Vec<Packet>> {
    let (s, bitlen) = map(bits(15), bitvec_to_num)(s)?;
    let (_, children) = many0(p_packet)(&s[0..bitlen])?;
    Ok((&s[bitlen..], children))
}

fn children_upto_amount<'a>(s: &'a str) -> IResult<&'a str, Vec<Packet>> {
    let (mut s, num) = map(bits(11), bitvec_to_num)(s)?;

    let mut children = vec![];

    let mut child: Packet;
    while children.len() < num {
        (s, child) = p_packet(s)?;
        children.push(child);
    }

    Ok((s, children))
}

fn p_operator(s: &str) -> IResult<&str, Packet> {
    map(
        tuple((bits_value(3), bits_neq_value(3, LITERAL), packet_children)),
        |(version, type_id, children)| Packet {
            version,
            type_id,
            number: 0,
            children,
        },
    )(s)
}

fn p_packet(s: &str) -> IResult<&str, Packet> {
    alt((p_literal, p_operator))(s)
}

pub fn parse_packet(s: &str) -> Packet {
    p_packet(s).unwrap().1
}

pub fn hextobin(s: &str) -> String {
    s.chars()
        .map(|c| {
            format!(
                "{:01$b}",
                if c.is_numeric() {
                    (c as usize) - ('0' as usize)
                } else {
                    (c as usize) - ('A' as usize) + 10
                },
                4
            )
        })
        .collect::<Vec<_>>()
        .join("")
}

#[test]
fn test_hextobin() {
    assert_eq!(
        hextobin("EE00D40C823060"),
        "11101110000000001101010000001100100000100011000001100000".to_string()
    );
}

#[test]
fn test_parse() {
    assert_eq!(
        p_literal("110100101111111000101000"),
        Ok((
            "000",
            Packet {
                version: 6,
                type_id: LITERAL,
                number: 2021,
                children: vec![]
            }
        ))
    );

    assert_eq!(
        p_operator("00111000000000000110111101000101001010010001001000000000"),
        Ok((
            "0000000",
            Packet {
                version: 1,
                type_id: 6,
                number: 0,
                children: vec![
                    Packet {
                        version: 6,
                        type_id: LITERAL,
                        number: 10,
                        children: vec![]
                    },
                    Packet {
                        version: 2,
                        type_id: LITERAL,
                        number: 20,
                        children: vec![]
                    }
                ]
            }
        ))
    );

    assert_eq!(
        p_operator("11101110000000001101010000001100100000100011000001100000"),
        Ok((
            "00000",
            Packet {
                version: 7,
                type_id: 3,
                number: 0,
                children: vec![
                    Packet {
                        version: 2,
                        type_id: LITERAL,
                        number: 1,
                        children: vec![]
                    },
                    Packet {
                        version: 4,
                        type_id: LITERAL,
                        number: 2,
                        children: vec![]
                    },
                    Packet {
                        version: 1,
                        type_id: LITERAL,
                        number: 3,
                        children: vec![]
                    }
                ]
            }
        ))
    );

    assert_eq!(
        parse_packet("11101110000000001101010000001100100000100011000001100000"),
        Packet {
            version: 7,
            type_id: 3,
            number: 0,
            children: vec![
                Packet {
                    version: 2,
                    type_id: LITERAL,
                    number: 1,
                    children: vec![]
                },
                Packet {
                    version: 4,
                    type_id: LITERAL,
                    number: 2,
                    children: vec![]
                },
                Packet {
                    version: 1,
                    type_id: LITERAL,
                    number: 3,
                    children: vec![]
                }
            ]
        }
    );
}
