use crate::util::parsers::unsigned_number;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::error::Error;

type GeneratorOutput = Vec<u32>;
type PartInput = [u32];

#[aoc_generator(day4)]
pub fn generator(input: &[u8]) -> Result<GeneratorOutput, Box<dyn Error>> {
    use nom::{bytes::complete::tag, combinator::all_consuming, multi::separated_list};
    Ok(
        all_consuming(separated_list(tag(b"-"), unsigned_number::<u32>))(input)
            .map_err(|err| format!("Parser error: {:x?}", err))?
            .1,
    )
}

fn digits(n: u32) -> Vec<u32> {
    fn digits_inner(n: u32, xs: &mut Vec<u32>) {
        if n >= 10 {
            digits_inner(n / 10, xs);
        }
        xs.push(n % 10);
    }
    let mut xs = Vec::new();
    digits_inner(n, &mut xs);
    xs
}

fn valid_password(pw: u32) -> bool {
    let digits = digits(pw);
    let mut monotonous = true;
    let mut pairs = 0;
    for (&d0, &d1) in digits.iter().zip(digits.iter().skip(1)) {
        if d0 > d1 {
            monotonous = false;
            break;
        } else if d0 == d1 {
            pairs += 1;
        }
    }
    monotonous && pairs >= 1
}

fn valid_password_2(pw: u32) -> bool {
    let digits = digits(pw);
    let counts = digits.iter().fold(HashMap::new(), |mut map, digit| {
        map.entry(*digit).and_modify(|v| *v += 1).or_insert(1u32);
        map
    });
    valid_password(pw) && counts.values().find(|v| **v == 2).is_some()
}

#[aoc(day4, part1)]
pub fn part_1(input: &PartInput) -> usize {
    (input[0]..=input[1])
        .filter(|pw| valid_password(*pw))
        .count()
}

#[aoc(day4, part2)]
pub fn part_2(input: &PartInput) -> usize {
    (input[0]..=input[1])
        .filter(|pw| valid_password_2(*pw))
        .count()
}
