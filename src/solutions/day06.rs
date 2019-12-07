use aoc_runner_derive::{aoc, aoc_generator};
use nom::lib::std::iter::successors;
use nom::IResult;
use smallvec::SmallVec;
use std::mem::swap;
use std::{collections::HashMap, error::Error};

type GeneratorOutput = Vec<(Identifier, Identifier)>;
type PartInput = [(Identifier, Identifier)];

type Identifier = [u8; 3];

fn parse_identifier(input: &[u8]) -> IResult<&[u8], Identifier> {
    use nom::bytes::complete::take;
    let mut result = [0u8; 3];
    let (input, chars) = take(3usize)(input)?;
    unsafe {
        std::ptr::copy_nonoverlapping(chars.as_ptr(), result.as_mut_ptr(), 3);
    }
    Ok((input, result))
}

#[aoc_generator(day6)]
pub fn generator(input: &[u8]) -> Result<GeneratorOutput, Box<dyn Error>> {
    use nom::{
        bytes::complete::tag, combinator::all_consuming, multi::separated_list,
        sequence::separated_pair,
    };
    Ok(all_consuming(separated_list(
        tag(b"\n"),
        separated_pair(parse_identifier, tag(b")"), parse_identifier),
    ))(input)
    .map_err(|err| format!("Parser error: {:x?}", err))?
    .1)
}

type OrbitedBy = HashMap<Identifier, SmallVec<[Identifier; 2]>>;

fn create_orbited_by(edges: &[(Identifier, Identifier)]) -> OrbitedBy {
    edges.iter().fold(
        HashMap::<_, SmallVec<[_; 2]>>::with_capacity(edges.len()),
        |mut acc, (to, from)| {
            acc.entry(*to)
                .and_modify(|children| children.push(*from))
                .or_insert_with(|| smallvec!(*from));
            acc
        },
    )
}

#[aoc(day6, part1)]
pub fn part_1(input: &PartInput) -> u64 {
    let orbited_by = create_orbited_by(input);
    let root = *b"COM";

    let mut sum = 0u64;
    let mut this_layer = Vec::with_capacity(32);
    let mut next_layer = Vec::with_capacity(32);
    this_layer.push(root);
    for depth in 0u64.. {
        for node in this_layer.iter() {
            sum += depth;
            if let Some(children) = orbited_by.get(node) {
                next_layer.extend(children.iter().cloned());
            }
        }
        if next_layer.is_empty() {
            break;
        }
        swap(&mut this_layer, &mut next_layer);
        next_layer.clear();
    }

    sum
}

#[aoc(day6, part1, recursive)]
pub fn part_1_recursive(input: &PartInput) -> u64 {
    let orbited_by = create_orbited_by(input);
    let root = *b"COM";

    fn orbit_count(orbited_by: &OrbitedBy, node: Identifier, depth: u64) -> u64 {
        match orbited_by.get(&node) {
            Some(children) => {
                children
                    .iter()
                    .map(|child| orbit_count(orbited_by, *child, depth + 1))
                    .sum::<u64>()
                    + depth
            }
            None => depth,
        }
    }

    orbit_count(&orbited_by, root, 0)
}

#[aoc(day6, part2)]
pub fn part_2(input: &PartInput) -> usize {
    let orbits = input
        .iter()
        .map(|(to, from)| (*from, *to))
        .collect::<HashMap<_, _>>();

    let start = *b"YOU";
    let goal = *b"SAN";

    let parent_node = |node: &Identifier| orbits.get(node).cloned();

    let start_parents = successors(Some(start), parent_node)
        .enumerate()
        .map(|(i, node)| (node, i))
        .collect::<HashMap<_, _>>();

    successors(Some(goal), parent_node)
        .enumerate()
        .find_map(|(i, node)| start_parents.get(&node).map(|cost| *cost + i - 2))
        .unwrap()
}

/// # Panics
/// This function panics should `start` or `goal` be the root of the tree
#[aoc(day6, part2, in_place)]
pub fn part_2_in_place(input: &PartInput) -> usize {
    let mut orbits: HashMap<Identifier, (Identifier, Option<usize>)> = input
        .iter()
        .map(|(to, from)| (*from, (*to, None)))
        .collect::<HashMap<_, _>>();

    let start = *b"YOU";
    let goal = *b"SAN";

    let mut current_node = start;
    let mut step_count = 0usize;
    while let Some((node, cost)) = orbits.get_mut(&current_node) {
        *cost = Some(step_count);
        current_node = *node;
        step_count += 1;
    }

    if let Some((_, Some(cost))) = orbits.get(&goal) {
        return *cost;
    }
    successors(orbits.get(&goal), |(node, _)| orbits.get(node))
        .enumerate()
        .find_map(|(i, (_, cost))| cost.map(|cost| cost + i - 2))
        .unwrap()
}
