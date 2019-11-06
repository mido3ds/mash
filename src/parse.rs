use nom::*;
use nom::character::is_alphabetic;
use nom::character::is_alphanumeric;
use nom::branch::alt;
use nom::sequence::pair;
use nom::bytes::complete::tag;
use nom::bytes::complete::

fn name(s: &str) -> IResult<&str, &str> {
    alt((is_alphabetic, pair(name, is_alphabetic), pair(name, tag)))
}
