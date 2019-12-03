use crate::util::parsers::unsigned_number;
use aoc_runner_derive::{aoc, aoc_generator};
use simd_aligned::{u32s, VectorD};
use std::error::Error;

type GeneratorOutput = Vec<u32>;
type PartInput = [u32];

#[aoc_generator(day1)]
pub fn generator(input: &[u8]) -> Result<GeneratorOutput, Box<dyn Error>> {
    use nom::{bytes::complete::tag, combinator::all_consuming, multi::separated_list};
    Ok(
        all_consuming(separated_list(tag(b"\n"), unsigned_number::<u32>))(input)
            .map_err(|err| format!("Parser error: {:x?}", err))?
            .1,
    )
}

fn fuel_requirement(weight: i32) -> i32 {
    weight / 3 - 2
}

#[aoc(day1, part1)]
pub fn part_1(input: &PartInput) -> u32 {
    input
        .iter()
        .map(|weight| fuel_requirement(*weight as i32))
        .sum::<i32>() as u32
}

#[aoc(day1, part2)]
pub fn part_2(input: &PartInput) -> u32 {
    fn recursive_fuel_requirement(weight: i32) -> i32 {
        let initial_weight = fuel_requirement(weight);
        let mut total_weight = initial_weight;
        let mut last_added_weight = initial_weight;
        loop {
            let added_fuel = fuel_requirement(last_added_weight);
            if added_fuel <= 0 {
                return total_weight;
            }
            total_weight += added_fuel;
            last_added_weight = added_fuel;
        }
    }

    input
        .iter()
        .map(|weight| recursive_fuel_requirement(*weight as i32))
        .sum::<i32>() as u32
}

// saturating sub optimizations

fn fuel_requirement_saturating(weight: u32) -> u32 {
    (weight / 3).saturating_sub(2)
}

#[aoc(day1, part2, recursive)]
pub fn part_2_recursive(input: &PartInput) -> u32 {
    fn recursive_fuel_requirement(weight: u32) -> u32 {
        match fuel_requirement_saturating(weight) {
            0 => 0,
            fuel => fuel + recursive_fuel_requirement(fuel),
        }
    }

    input
        .iter()
        .map(|weight| recursive_fuel_requirement(*weight))
        .sum()
}

#[aoc(day1, part2, iterative)]
pub fn part_2_iterative(input: &PartInput) -> u32 {
    fn recursive_fuel_requirement(weight: u32) -> u32 {
        let mut total_fuel = 0;
        let mut added_weight = weight;
        loop {
            added_weight = fuel_requirement_saturating(added_weight);
            if added_weight == 0 {
                return total_fuel;
            }
            total_fuel += added_weight;
        }
    }

    input
        .iter()
        .map(|weight| recursive_fuel_requirement(*weight))
        .sum()
}

// SIMD

fn fuel_requirement_simd(weight: &u32s) -> u32s {
    (*weight / 3).max(u32s::splat(2)) - 2
}

#[aoc(day1, part1, simd)]
pub fn part_1_simd(input: &PartInput) -> u32 {
    let mut weights = VectorD::<u32s>::with(0u32, input.len());
    for (weight_in, weight_out) in input.iter().zip(weights.flat_mut().iter_mut()) {
        *weight_out = *weight_in;
    }
    weights
        .iter()
        .map(fuel_requirement_simd)
        .sum::<u32s>()
        .wrapping_sum()
}

#[aoc(day1, part2, simd)]
pub fn part_2_simd(input: &PartInput) -> u32 {
    fn recursive_fuel_requirement(weight: &u32s) -> u32s {
        let mut total_fuel = u32s::splat(0);
        let mut added_weight = *weight;
        loop {
            added_weight = fuel_requirement_simd(&added_weight);
            if added_weight == u32s::splat(0) {
                return total_fuel;
            }
            total_fuel += added_weight;
        }
    }
    let mut weights = VectorD::<u32s>::with(0u32, input.len());
    for (weight_in, weight_out) in input.iter().zip(weights.flat_mut().iter_mut()) {
        *weight_out = *weight_in;
    }
    weights
        .iter()
        .map(recursive_fuel_requirement)
        .sum::<u32s>()
        .wrapping_sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn part_1_tests(f: fn(input: &PartInput) -> u32) {
        assert_eq!(f(&[12]), 2);
        assert_eq!(f(&[14]), 2);
        assert_eq!(f(&[1969]), 654);
        assert_eq!(f(&[100756]), 33583);
    }

    #[test]
    fn part_1() {
        part_1_tests(super::part_1);
    }

    #[test]
    fn part_1_simd() {
        part_1_tests(super::part_1_simd);
    }

    fn part_2_tests(f: fn(input: &PartInput) -> u32) {
        assert_eq!(f(&[14]), 2);
        assert_eq!(f(&[1969]), 966);
        assert_eq!(f(&[100756]), 50346);
    }

    #[test]
    fn part_2() {
        part_2_tests(super::part_2);
    }

    #[test]
    fn part_2_simd() {
        part_2_tests(super::part_2_simd);
    }

    #[test]
    fn part_2_iterative() {
        part_2_tests(super::part_2_iterative);
    }

    #[test]
    fn part_2_recursive() {
        part_2_tests(super::part_2_recursive);
    }
}
