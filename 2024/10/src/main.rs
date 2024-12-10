use anyhow::{Context, Result};
use petgraph::algo::DfsSpace;
use petgraph::prelude::DiGraphMap;
use rayon::iter::{ParallelBridge, ParallelIterator};

const INPUT: &str = include_str!("./input");

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    const DIRECTIONS: [Direction; 4] =
        [Direction::Up, Direction::Right, Direction::Down, Direction::Left];
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Location {
    x: usize,
    y: usize,
}

impl Location {
    fn adjacent(self, direction: Direction) -> Option<Location> {
        let Location { x, y } = self;

        let (x, y) = match direction {
            Direction::Up => y.checked_sub(1).map(|y| (x, y)),
            Direction::Right => x.checked_add(1).map(|x| (x, y)),
            Direction::Down => y.checked_add(1).map(|y| (x, y)),
            Direction::Left => x.checked_sub(1).map(|x| (x, y)),
        }?;

        Some(Location { x, y })
    }
}

#[derive(Clone, Copy, Debug)]
struct Node {
    location: Location,
    height: u8,
}

struct Map {
    nodes: Vec<Vec<u8>>,
}

impl Map {
    fn nodes(&self) -> impl Iterator<Item = Node> + '_ {
        self.nodes.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .copied()
                .enumerate()
                .map(move |(x, height)| Node { location: Location { x, y }, height })
        })
    }

    fn get(&self, location: Location) -> Option<Node> {
        self.nodes
            .get(location.y)
            .and_then(|row| row.get(location.x))
            .copied()
            .map(|height| Node { location, height })
    }

    fn trail_heads(&self) -> impl Iterator<Item = Node> + '_ {
        self.nodes().filter(|node| node.height == 0)
    }

    fn trail_tails(&self) -> impl Iterator<Item = Node> + '_ {
        self.nodes().filter(|node| node.height == 9)
    }
}

type Graph = DiGraphMap<Location, ()>;

fn main() -> Result<()> {
    let map = parse_input(INPUT).context("failed to parse input")?;

    let mut graph = Graph::new();

    for node in map.nodes() {
        graph.add_node(node.location);

        let adjacent = Direction::DIRECTIONS
            .into_iter()
            .filter_map(|dir| node.location.adjacent(dir))
            .filter_map(|loc| map.get(loc));

        for adj in adjacent {
            if adj.height.checked_sub(1) == Some(node.height) {
                graph.add_edge(node.location, adj.location, ());
            }
        }
    }

    part1(&map, &graph);
    part2(&map, &graph);

    Ok(())
}

fn part1(map: &Map, graph: &Graph) {
    let mut space = DfsSpace::new(&graph);

    let sum = map
        .trail_heads()
        .map(move |head| {
            map.trail_tails()
                .filter(|tail| {
                    petgraph::algo::has_path_connecting(
                        &graph,
                        head.location,
                        tail.location,
                        Some(&mut space),
                    )
                })
                .count()
        })
        .sum::<usize>();

    println!("part1: {sum}");
}

fn part2(map: &Map, graph: &Graph) {
    struct Marker;

    impl<A> FromIterator<A> for Marker {
        fn from_iter<T: IntoIterator<Item = A>>(_iter: T) -> Self {
            Marker
        }
    }

    let sum = map
        .trail_heads()
        .par_bridge()
        .map(|head| {
            map.trail_tails()
                .par_bridge()
                .map(|tail| {
                    petgraph::algo::all_simple_paths::<Marker, _>(
                        graph,
                        head.location,
                        tail.location,
                        0,
                        None,
                    )
                    .count()
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("part2: {sum}");
}

fn parse_input(input: &str) -> Result<Map> {
    input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    c.to_digit(10)
                        .and_then(|d| u8::try_from(d).ok())
                        .with_context(|| format!("invalid character at ({row}, {col})"))
                })
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()
        .map(|nodes| Map { nodes })
}
