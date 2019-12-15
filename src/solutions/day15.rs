use crate::util::intcode::{parse_intcode_text, Emulator};
use aoc_runner_derive::{aoc, aoc_generator};
use arrayvec::ArrayVec;
use nalgebra::{Point2, Vector2};
use petgraph::{
    algo::{astar, dijkstra},
    graph::{DefaultIx, NodeIndex},
    Graph, Undirected,
};
use std::{collections::HashMap, error::Error};

type Word = i64;
type GeneratorOutput = Vec<Word>;
type PartInput = [Word];

#[aoc_generator(day15)]
pub fn generator(input: &[u8]) -> Result<GeneratorOutput, Box<dyn Error>> {
    parse_intcode_text(input)
}

fn to_intcode_direction(vec: &Vector2<i64>) -> i64 {
    match (vec.x, vec.y) {
        (0, 1) => 1,
        (0, -1) => 2,
        (1, 0) => 3,
        (-1, 0) => 4,
        _ => panic!(),
    }
}

fn manhattan_distance(a: &Point2<i64>, b: &Point2<i64>) -> i64 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn cardinal_directions() -> impl Iterator<Item = Vector2<i64>> {
    ArrayVec::from([
        Vector2::new(0, 1),
        Vector2::new(0, -1),
        Vector2::new(1, 0),
        Vector2::new(-1, 0),
    ])
    .into_iter()
}

fn search_path(
    graph: &Graph<Point2<i64>, (), Undirected, DefaultIx>,
    map_elements: &HashMap<Point2<i64>, Option<NodeIndex>>,
    start: &Point2<i64>,
    end: &Point2<i64>,
) -> Option<(i64, Vec<NodeIndex>)> {
    astar(
        &graph,
        map_elements.get(start).unwrap().unwrap(),
        |node| node == map_elements.get(end).unwrap().unwrap(),
        |_| 1i64,
        |node| manhattan_distance(end, graph.node_weight(node).unwrap()),
    )
}

fn build_map(
    program: &[Word],
) -> (
    Graph<Point2<i64>, (), Undirected, DefaultIx>,
    HashMap<Point2<i64>, Option<NodeIndex>>,
    Option<Point2<i64>>,
) {
    let mut control_program = Emulator::new(program.to_vec());
    let mut position = Point2::<i64>::new(0, 0);
    let mut path_graph = Graph::<Point2<i64>, (), Undirected, DefaultIx>::default();
    let mut map_elements = HashMap::<Point2<i64>, Option<NodeIndex>>::new();
    let mut oxygen_tank_position = None;
    let mut open_list = Vec::<(Point2<i64>, Vector2<i64>)>::new();

    let first_node = path_graph.add_node(position.clone());
    map_elements.insert(position.clone(), Some(first_node));
    for direction in cardinal_directions() {
        open_list.push((position.clone(), direction));
    }

    while let Some((target_position, target_direction)) = open_list.pop() {
        if map_elements.contains_key(&(target_position + target_direction)) {
            continue;
        }

        // move to next open element
        if position != target_position {
            let (_, path) =
                search_path(&path_graph, &map_elements, &position, &target_position).unwrap();
            for target_node in path.into_iter().skip(1) {
                let next_direction = path_graph.node_weight(target_node).unwrap() - position;
                control_program.push_input(to_intcode_direction(&next_direction));
                control_program.run();
                position += next_direction;
            }
        }

        // inspect open element
        control_program.push_input(to_intcode_direction(&target_direction));
        let bot_status = control_program.run().into_option().unwrap();
        match bot_status {
            0 => {
                map_elements.insert(target_position + target_direction, None);
            }
            1 | 2 => {
                position += target_direction;
                if bot_status == 2 {
                    oxygen_tank_position = Some(position.clone());
                }
                let new_node = path_graph.add_node(position.clone());
                map_elements.insert(position.clone(), Some(new_node));
                for direction in cardinal_directions() {
                    if let Some(Some(other)) = map_elements.get(&(position + direction)) {
                        path_graph.add_edge(new_node, *other, ());
                    } else {
                        open_list.push((position.clone(), direction.clone()));
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    (path_graph, map_elements, oxygen_tank_position)
}

#[aoc(day15, part1)]
pub fn part_1(input: &PartInput) -> i64 {
    let (path_graph, map_elements, oxygen_tank_position) = build_map(input);

    let (cost, _) = search_path(
        &path_graph,
        &map_elements,
        &Point2::new(0, 0),
        &oxygen_tank_position.unwrap(),
    )
    .unwrap();
    cost
}

#[aoc(day15, part2)]
pub fn part_2(input: &PartInput) -> i64 {
    let (path_graph, map_elements, oxygen_tank_position) = build_map(input);

    dijkstra(
        &path_graph,
        map_elements
            .get(&oxygen_tank_position.unwrap())
            .unwrap()
            .unwrap(),
        None,
        |_| 1,
    )
    .into_iter()
    .map(|(_, cost)| cost)
    .max()
    .unwrap()
}
