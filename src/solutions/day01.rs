use crate::util::parsers::unsigned_number;
use aoc_runner_derive::{aoc, aoc_generator};
use std::{collections::HashSet, error::Error};

type GeneratorOutput = Vec<i64>;
type PartInput = [i64];

#[aoc_generator(day1)]
pub fn generator(input: &[u8]) -> Result<GeneratorOutput, Box<dyn Error>> {
    use nom::{bytes::complete::tag, combinator::all_consuming, multi::separated_list};
    Ok(
        all_consuming(separated_list(tag(b"\n"), unsigned_number::<i64>))(input)
            .map_err(|err| format!("Parser error: {:x?}", err))?
            .1,
    )
}

#[aoc(day1, part1)]
pub fn part_1(input: &PartInput) -> i64 {
    input.iter().map(|weight| fuel_requirement(*weight)).sum()
}

fn fuel_requirement(weight: i64) -> i64 {
    weight / 3 - 2
}

fn recursive_fuel_requirement(weight: i64) -> i64 {
    let initial_weight = fuel_requirement(weight);
    let mut total_weight = initial_weight;
    let mut last_added_weight = initial_weight;
    loop {
        let added_fuel = dbg!(fuel_requirement(last_added_weight));
        if added_fuel <= 0 {
            return total_weight;
        }
        total_weight += added_fuel;
        last_added_weight = added_fuel;
    }
}

#[aoc(day1, part2)]
pub fn part_2(input: &PartInput) -> i64 {
    input
        .iter()
        .map(|weight| recursive_fuel_requirement(*weight))
        .sum()
}
