use aoc_runner_derive::{aoc, aoc_generator};
use indexmap::map::IndexMap;
use nom::IResult;
use std::{collections::HashMap, error::Error};

pub type Chemical = Vec<u8>;
#[derive(Debug, Clone)]
pub struct Reaction {
    output: (Chemical, usize),
    inputs: Vec<(Chemical, usize)>,
}

type GeneratorOutput = Vec<Reaction>;
type PartInput = [Reaction];

pub fn parse_chemical(input: &[u8]) -> IResult<&[u8], (Chemical, usize)> {
    use crate::util::parsers::unsigned_number;
    use nom::bytes::complete::{tag, take_while};
    let (input, count) = unsigned_number::<usize>(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, chemical) = take_while(|b: u8| b.is_ascii_uppercase())(input)?;
    Ok((input, (chemical.to_owned(), count)))
}

pub fn parse_reaction(input: &[u8]) -> IResult<&[u8], Reaction> {
    use nom::{bytes::complete::tag, multi::separated_list};
    let (input, input_chemicals) = separated_list(tag(b", "), parse_chemical)(input)?;
    let (input, _) = tag(" => ")(input)?;
    let (input, output_chemical) = parse_chemical(input)?;
    Ok((
        input,
        Reaction {
            output: output_chemical,
            inputs: input_chemicals,
        },
    ))
}

#[aoc_generator(day14)]
pub fn generator(input: &[u8]) -> Result<GeneratorOutput, Box<dyn Error>> {
    use nom::{bytes::complete::tag, combinator::all_consuming, multi::separated_list};
    Ok(
        all_consuming(separated_list(tag(b"\n"), parse_reaction))(input)
            .map_err(|err| format!("Parser error: {:x?}", err))?
            .1,
    )
}

#[aoc(day14, part1)]
pub fn part_1(input: &PartInput) -> usize {
    let dependency_map = input
        .iter()
        .map(|reaction| (reaction.output.0.clone(), reaction.clone()))
        .collect::<HashMap<_, _>>();
    let mut current_required_chemicals = IndexMap::new();
    current_required_chemicals.insert(b"FUEL".to_vec(), 1usize);
    let mut leftovers = HashMap::<Chemical, usize>::new();
    let mut ore_required = 0usize;

    while let Some((required_chemical, mut required_count)) = current_required_chemicals.pop() {
        leftovers
            .entry(required_chemical.clone())
            .and_modify(|leftover_count| {
                let sub_amt = (*leftover_count).min(required_count);
                required_count -= sub_amt;
                *leftover_count -= sub_amt;
            });
        if required_count > 0 {
            let reaction = dependency_map.get(&required_chemical).unwrap();
            let batch_count = (required_count - 1 + reaction.output.1) / reaction.output.1;
            let made_count = batch_count * reaction.output.1;
            for (input_chemical, input_count) in reaction.inputs.iter() {
                if input_chemical == b"ORE" {
                    ore_required += input_count * batch_count;
                } else {
                    *current_required_chemicals
                        .entry(input_chemical.clone())
                        .or_insert(0) += input_count * batch_count;
                }
            }
            *leftovers.entry(required_chemical).or_insert(0) += made_count - required_count;
        }
    }

    ore_required
}

#[aoc(day14, part2)]
pub fn part_2(input: &PartInput) -> usize {
    let dependency_map = input
        .iter()
        .map(|reaction| (reaction.output.0.clone(), reaction.clone()))
        .collect::<HashMap<_, _>>();
    let mut current_required_chemicals = IndexMap::new();
    current_required_chemicals.insert(b"FUEL".to_vec(), 6_700_000usize);
    let mut leftovers = HashMap::<Chemical, usize>::new();
    let mut ore_required = 0usize;
    let mut fuel_made = 6_700_000usize;

    'outer: loop {
        *current_required_chemicals
            .entry(b"FUEL".to_vec())
            .or_insert(0) += 1usize;
        while let Some((required_chemical, mut required_count)) = current_required_chemicals.pop() {
            leftovers
                .entry(required_chemical.clone())
                .and_modify(|leftover_count| {
                    let sub_amt = (*leftover_count).min(required_count);
                    required_count -= sub_amt;
                    *leftover_count -= sub_amt;
                });
            if required_count > 0 {
                let reaction = dependency_map.get(&required_chemical).unwrap();
                let batch_count = (required_count - 1 + reaction.output.1) / reaction.output.1;
                let made_count = batch_count * reaction.output.1;
                for (input_chemical, input_count) in reaction.inputs.iter() {
                    if input_chemical == b"ORE" {
                        ore_required += input_count * batch_count;
                        if ore_required > 1_000_000_000_000 {
                            break 'outer;
                        }
                    } else {
                        *current_required_chemicals
                            .entry(input_chemical.clone())
                            .or_insert(0) += input_count * batch_count;
                    }
                }
                *leftovers.entry(required_chemical).or_insert(0) += made_count - required_count;
            }
        }
        fuel_made += 1;
    }

    fuel_made
}
