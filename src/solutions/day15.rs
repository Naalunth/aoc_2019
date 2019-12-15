use crate::util::intcode::{parse_intcode_text, Emulator};
use aoc_runner_derive::{aoc, aoc_generator};
use arrayvec::ArrayVec;
use fixedbitset::FixedBitSet;
use nalgebra::{Point2, Vector2};
use petgraph::{
    algo::astar,
    graph::{DefaultIx, NodeIndex},
    visit::{VisitMap, Visitable},
    Graph, Undirected,
};
use std::{collections::HashMap, error::Error};

type Word = i32;
type GeneratorOutput = Vec<Word>;
type PartInput = [Word];

#[aoc_generator(day15)]
pub fn generator(input: &[u8]) -> Result<GeneratorOutput, Box<dyn Error>> {
    parse_intcode_text(input)
}

type Point = Point2<i32>;
type Vector = Vector2<i32>;
type PathGraph = Graph<Point, (), Undirected, DefaultIx>;
type Map = HashMap<Point, Option<NodeIndex>>;

fn intcode_direction(vec: Vector) -> i32 {
    match (vec.x, vec.y) {
        (0, 1) => 1,
        (0, -1) => 2,
        (1, 0) => 3,
        (-1, 0) => 4,
        _ => panic!(),
    }
}

fn manhattan_distance(a: Point, b: Point) -> u32 {
    ((a.x - b.x).abs() + (a.y - b.y).abs()) as u32
}

fn cardinal_directions() -> impl Iterator<Item = Vector> {
    ArrayVec::from([[0, 1], [0, -1], [1, 0], [-1, 0]])
        .into_iter()
        .map(Into::into)
}

fn dft(
    position: Point,
    node: NodeIndex,
    controller: &mut Emulator<Word>,
    graph: &mut PathGraph,
    map: &mut Map,
    oxygen_pos: &mut Option<Point>,
) {
    for direction in cardinal_directions() {
        let neighbor = position + direction;

        if let Some(element) = map.get(&neighbor) {
            if let &Some(other_node) = element {
                graph.add_edge(node, other_node, ());
            }
            continue;
        }

        controller.push_input(intcode_direction(direction));
        match controller.run().into_option().unwrap() {
            0 => {
                map.insert(neighbor, None);
            }
            bot_status => {
                if bot_status == 2 {
                    *oxygen_pos = Some(neighbor);
                }
                let new_node = graph.add_node(neighbor);
                map.insert(neighbor, Some(new_node));

                dft(neighbor, new_node, controller, graph, map, oxygen_pos);
                controller.push_input(intcode_direction(-direction));
                controller.run();
            }
        }
    }
}

fn build_map(program: &[Word]) -> (PathGraph, Map, Point) {
    let mut controller = Emulator::new(program.to_vec());
    let position = [0, 0].into();
    let mut graph = Graph::default();
    let mut map = HashMap::default();
    let mut oxygen_pos = None;

    let first_node = graph.add_node(position);
    map.insert(position, Some(first_node));
    dft(
        position,
        first_node,
        &mut controller,
        &mut graph,
        &mut map,
        &mut oxygen_pos,
    );

    (graph, map, oxygen_pos.unwrap())
}

#[aoc(day15, part1)]
pub fn part_1(input: &PartInput) -> u32 {
    let (graph, map, oxygen_pos) = build_map(input);

    let (cost, _) = astar(
        &graph,
        map[&[0, 0].into()].unwrap(),
        |node| node == map[&oxygen_pos].unwrap(),
        |_| 1,
        |node| manhattan_distance(oxygen_pos, graph[node]),
    )
    .unwrap();
    cost
}

#[aoc(day15, part2)]
pub fn part_2(input: &PartInput) -> u32 {
    let (graph, map, oxygen_pos) = build_map(input);

    fn depth(graph: &PathGraph, node: NodeIndex, v: &mut FixedBitSet, d: u32) -> Option<u32> {
        v.visit(node).then_with(|| {
            graph
                .neighbors(node)
                .flat_map(|n| depth(graph, n, v, d + 1))
                .max()
                .unwrap_or(d)
        })
    }
    depth(&graph, map[&oxygen_pos].unwrap(), &mut graph.visit_map(), 0).unwrap()
}
