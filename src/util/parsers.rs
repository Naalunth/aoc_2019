#![allow(dead_code)]
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{map_res, opt, recognize},
    sequence::preceded,
    IResult,
};
use num_traits::{CheckedAdd, CheckedMul, CheckedSub, FromPrimitive, Zero};

pub fn unsigned_number<T>(input: &[u8]) -> IResult<&[u8], T>
where
    T: FromPrimitive + Zero + CheckedAdd + CheckedMul,
{
    map_res(take_digit_bytes, |s: &[u8]| btoi::btou::<T>(s))(input)
}

pub fn signed_number<T>(input: &[u8]) -> IResult<&[u8], T>
where
    T: FromPrimitive + Zero + CheckedAdd + CheckedSub + CheckedMul,
{
    map_res(
        recognize(preceded(opt(alt((tag(b"-"), tag(b"+")))), take_digit_bytes)),
        |s: &[u8]| btoi::btoi::<T>(s),
    )(input)
}

fn take_digit_bytes(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while1(|c: u8| c.is_ascii_digit())(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsigned_number() {
        assert_eq!(
            unsigned_number::<u8>(b"42abc").unwrap(),
            (&b"abc"[..], 42u8)
        );
        assert!(unsigned_number::<u8>(b"abc42").is_err());
    }

    #[test]
    fn test_signed_number() {
        assert_eq!(signed_number::<i8>(b"42abc").unwrap(), (&b"abc"[..], 42i8));
        assert_eq!(signed_number::<i8>(b"+42abc").unwrap(), (&b"abc"[..], 42i8));
        assert_eq!(
            signed_number::<i8>(b"-42abc").unwrap(),
            (&b"abc"[..], -42i8)
        );
        assert!(signed_number::<i8>(b"-bc42").is_err());
        assert!(signed_number::<i8>(b"+bc42").is_err());
        assert!(signed_number::<i8>(b"abc42").is_err());
    }
}
