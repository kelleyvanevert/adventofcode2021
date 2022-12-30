use crate::Snum;
use nom::{
    branch::alt,
    character::complete::{char, u32},
    combinator::map,
    sequence::tuple,
    IResult,
};

fn p_snum(s: &str) -> IResult<&str, Snum> {
    alt((
        map(u32, |n| Snum::Reg(n as usize)),
        map(
            tuple((char('['), p_snum, char(','), p_snum, char(']'))),
            |(_, le, _, ri, _)| Snum::Pair(Box::new(le), Box::new(ri)),
        ),
    ))(s)
}

pub fn parse_snum(s: &str) -> Snum {
    p_snum(s).unwrap().1
}

#[test]
fn test_parse() {
    assert_eq!(p_snum("2]"), Ok(("]", Snum::Reg(2))));
    assert_eq!(
        p_snum("[24,[1,2]]-"),
        Ok((
            "-",
            Snum::Pair(
                Box::new(Snum::Reg(24)),
                Box::new(Snum::Pair(Box::new(Snum::Reg(1)), Box::new(Snum::Reg(2))))
            )
        ))
    );

    assert_eq!(
        parse_snum("[24,[1,2]]-"),
        Snum::Pair(
            Box::new(Snum::Reg(24)),
            Box::new(Snum::Pair(Box::new(Snum::Reg(1)), Box::new(Snum::Reg(2))))
        )
    );
}
