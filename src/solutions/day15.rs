use crate::util::intcode::{parse_intcode_text, Emulator};
use aoc_runner_derive::{aoc, aoc_generator};
use arrayvec::ArrayVec;
use itertools::Itertools;
use nalgebra::{Point2, Vector2};
use petgraph::algo::dijkstra;
use petgraph::{algo::astar, graph::DefaultIx, graph::NodeIndex, Graph, Undirected};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

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
    paths: &HashMap<Point2<i64>, NodeIndex>,
    start: &Point2<i64>,
    end: &Point2<i64>,
) -> Option<(i64, Vec<NodeIndex>)> {
    astar(
        &graph,
        *paths.get(start).unwrap(),
        |node| node == *paths.get(end).unwrap(),
        |_| 1i64,
        |node| manhattan_distance(end, graph.node_weight(node).unwrap()),
    )
}

#[aoc(day15, part1)]
pub fn part_1(input: &PartInput) -> i64 {
    let mut control_program = Emulator::new(input.to_vec());
    let mut open_list = Vec::<(Point2<i64>, Vector2<i64>)>::new();
    let mut position = Point2::<i64>::new(0, 0);
    let mut path_graph = Graph::<Point2<i64>, (), Undirected, DefaultIx>::default();
    let mut paths = HashMap::<Point2<i64>, NodeIndex>::new();
    let mut walls = HashSet::<Point2<i64>>::new();
    let mut oxygen_tank_position = None;

    for direction in cardinal_directions() {
        open_list.push((position.clone(), direction));
    }
    let first_node = path_graph.add_node(position.clone());
    paths.insert(position.clone(), first_node);

    while let Some((target_idx, _)) = open_list
        .iter()
        .enumerate()
        .min_by_key(|(_, (p, _))| manhattan_distance(&position, p))
    {
        let (target_position, target_direction) = open_list.swap_remove(target_idx);
        //        println!(
        //            "position: {:?}, target: {:?}, t_dir: {:?}",
        //            (position.x, position.y),
        //            (target_position.x, target_position.y),
        //            (target_direction.x, target_direction.y)
        //        );

        // move to inspect next open element
        if position != target_position {
            let (_, path) = search_path(&path_graph, &paths, &position, &target_position).unwrap();
            for target_node in path.into_iter().skip(1) {
                //                println!(
                //                    "moving to position: {:?}",
                //                    path_graph.node_weight(target_node)
                //                );
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
                walls.insert(target_position + target_direction);
            }
            1 | 2 => {
                position += target_direction;
                if bot_status == 2 {
                    oxygen_tank_position = Some(position.clone());
                }
                let new_node = path_graph.add_node(position.clone());
                paths.insert(position.clone(), new_node);
                for direction in cardinal_directions() {
                    if let Some(other) = paths.get(&(position + direction)) {
                        path_graph.add_edge(new_node, *other, ());
                    } else if walls.get(&(position + direction)).is_none() {
                        open_list.push((position.clone(), direction.clone()));
                    }
                }
            }
            _ => unreachable!(),
        }

        //        let (px_min, px_max) = paths.keys().map(|p| p.x).minmax().into_option().unwrap();
        //        let (py_min, py_max) = paths.keys().map(|p| p.y).minmax().into_option().unwrap();
        //        let (wx_min, wx_max) = walls.iter().map(|p| p.x).minmax().into_option().unwrap();
        //        let (wy_min, wy_max) = walls.iter().map(|p| p.y).minmax().into_option().unwrap();
        //        println!(
        //            "\n{}",
        //            ((py_min.min(wy_min) - 1)..=(py_max.max(wy_max) + 1))
        //                .rev()
        //                .map(|y| {
        //                    let paths = &paths;
        //                    let walls = &walls;
        //                    let position = &position;
        //                    ((px_min.min(wx_min) - 1)..=(px_max.max(wx_max) + 1))
        //                        .map(move |x| {
        //                            let point = Point2::new(x, y);
        //                            if *position == point {
        //                                "*"
        //                            } else if paths.get(&point).is_some() {
        //                                " "
        //                            } else if walls.get(&point).is_some() {
        //                                "█"
        //                            } else {
        //                                "░"
        //                            }
        //                        })
        //                        .format("")
        //                })
        //                .format("\n")
        //        )
    }

    let (cost, _) = search_path(
        &path_graph,
        &paths,
        &Point2::new(0, 0),
        &oxygen_tank_position.unwrap(),
    )
    .unwrap();
    cost
}

#[aoc(day15, part2)]
pub fn part_2(input: &PartInput) -> i64 {
    let mut control_program = Emulator::new(input.to_vec());
    let mut open_list = Vec::<(Point2<i64>, Vector2<i64>)>::new();
    let mut position = Point2::<i64>::new(0, 0);
    let mut path_graph = Graph::<Point2<i64>, (), Undirected, DefaultIx>::default();
    let mut paths = HashMap::<Point2<i64>, NodeIndex>::new();
    let mut walls = HashSet::<Point2<i64>>::new();
    let mut oxygen_tank_position = None;

    for direction in cardinal_directions() {
        open_list.push((position.clone(), direction));
    }
    let first_node = path_graph.add_node(position.clone());
    paths.insert(position.clone(), first_node);

    while let Some((target_idx, _)) = open_list
        .iter()
        .enumerate()
        .min_by_key(|(_, (p, _))| manhattan_distance(&position, p))
    {
        let (target_position, target_direction) = open_list.swap_remove(target_idx);
        //        println!(
        //            "position: {:?}, target: {:?}, t_dir: {:?}",
        //            (position.x, position.y),
        //            (target_position.x, target_position.y),
        //            (target_direction.x, target_direction.y)
        //        );

        // move to inspect next open element
        if position != target_position {
            let (_, path) = search_path(&path_graph, &paths, &position, &target_position).unwrap();
            for target_node in path.into_iter().skip(1) {
                //                println!(
                //                    "moving to position: {:?}",
                //                    path_graph.node_weight(target_node)
                //                );
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
                walls.insert(target_position + target_direction);
            }
            1 | 2 => {
                position += target_direction;
                if bot_status == 2 {
                    oxygen_tank_position = Some(position.clone());
                }
                let new_node = path_graph.add_node(position.clone());
                paths.insert(position.clone(), new_node);
                for direction in cardinal_directions() {
                    if let Some(other) = paths.get(&(position + direction)) {
                        path_graph.add_edge(new_node, *other, ());
                    } else if walls.get(&(position + direction)).is_none() {
                        open_list.push((position.clone(), direction.clone()));
                    }
                }
            }
            _ => unreachable!(),
        }

        //        println!("walls: {:?}", walls);
        //
        //        let (px_min, px_max) = paths.keys().map(|p| p.x).minmax().into_option().unwrap();
        //        let (py_min, py_max) = paths.keys().map(|p| p.y).minmax().into_option().unwrap();
        //        let (wx_min, wx_max) = walls.iter().map(|p| p.x).minmax().into_option().unwrap();
        //        let (wy_min, wy_max) = walls.iter().map(|p| p.y).minmax().into_option().unwrap();
        //        println!(
        //            "\n{}",
        //            ((py_min.min(wy_min) - 1)..=(py_max.max(wy_max) + 1))
        //                .rev()
        //                .map(|y| {
        //                    let paths = &paths;
        //                    let walls = &walls;
        //                    let position = &position;
        //                    ((px_min.min(wx_min) - 1)..=(px_max.max(wx_max) + 1))
        //                        .map(move |x| {
        //                            let point = Point2::new(x, y);
        //                            if *position == point {
        //                                "*"
        //                            } else if paths.get(&point).is_some() {
        //                                " "
        //                            } else if walls.get(&point).is_some() {
        //                                "█"
        //                            } else {
        //                                "░"
        //                            }
        //                        })
        //                        .format("")
        //                })
        //                .format("\n")
        //        )
    }

    dijkstra(
        &path_graph,
        *paths.get(&oxygen_tank_position.unwrap()).unwrap(),
        None,
        |_| 1,
    )
    .into_iter()
    .map(|(_, cost)| cost)
    .max()
    .unwrap()
}
