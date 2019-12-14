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

#[derive(Debug, Clone)]
pub struct CompactReaction {
    output: (u32, usize),
    inputs: Vec<(u32, usize)>,
}

fn build_dependency_map(input: &[Reaction]) -> HashMap<u32, CompactReaction> {
    let mut translation_map = HashMap::new();
    translation_map.insert(b"FUEL".to_vec(), 0u32);
    translation_map.insert(b"ORE".to_vec(), 1u32);
    let mut next_chem_key = 2u32;
    input
        .iter()
        .map(|reaction| {
            let mut get_key = |name| {
                *translation_map.entry(name).or_insert_with(|| {
                    let res = next_chem_key;
                    next_chem_key += 1;
                    res
                })
            };
            let cr = CompactReaction {
                output: (get_key(reaction.output.0.clone()), reaction.output.1),
                inputs: reaction
                    .inputs
                    .iter()
                    .map(|(name, count)| (get_key(name.clone()), *count))
                    .collect::<Vec<_>>(),
            };
            (get_key(reaction.output.0.clone()), cr)
        })
        .collect::<HashMap<_, _>>()
}

fn minimum_ore_cost(dependency_map: &HashMap<u32, CompactReaction>, fuel_amount: usize) -> usize {
    let mut current_required_chemicals = IndexMap::new();
    current_required_chemicals.insert(0, fuel_amount);
    let mut leftovers = HashMap::<u32, usize>::new();
    let mut ore_required = 0usize;

    while let Some((required_chemical, mut required_count)) = current_required_chemicals.pop() {
        leftovers
            .entry(required_chemical)
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
                if *input_chemical == 1 {
                    ore_required += input_count * batch_count;
                } else {
                    *current_required_chemicals
                        .entry(*input_chemical)
                        .or_insert(0) += input_count * batch_count;
                }
            }
            *leftovers.entry(required_chemical).or_insert(0) += made_count - required_count;
        }
    }

    ore_required
}

#[aoc(day14, part1)]
pub fn part_1(input: &PartInput) -> usize {
    minimum_ore_cost(&build_dependency_map(input), 1)
}

#[aoc(day14, part2)]
pub fn part_2(input: &PartInput) -> usize {
    let dependency_map = build_dependency_map(input);
    const GOAL_ORE_COST: usize = 1_000_000_000_000;
    let cost_of_one = minimum_ore_cost(&dependency_map, 1);

    // inclusive
    let mut fuel_minimum = GOAL_ORE_COST / cost_of_one;
    // exclusive
    let mut fuel_maximum = fuel_minimum * 2;

    // find upper bound
    loop {
        let cost = minimum_ore_cost(&dependency_map, fuel_maximum);
        if cost > GOAL_ORE_COST {
            break;
        }
        fuel_maximum *= 2;
    }

    // binary search
    loop {
        let fuel = (fuel_maximum + fuel_minimum) / 2;
        let cost = minimum_ore_cost(&dependency_map, fuel);
        if cost > GOAL_ORE_COST {
            fuel_maximum = fuel;
        } else if cost <= GOAL_ORE_COST {
            fuel_minimum = fuel;
        }
        if fuel_minimum + 1 == fuel_maximum {
            return fuel_minimum;
        }
    }
}
