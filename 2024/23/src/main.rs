#![feature(ascii_char)]

use std::ascii;
use std::collections::HashSet;

use anyhow::{Context, Result};
use petgraph::prelude::UnGraphMap;

const INPUT: &str = include_str!("./input");

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Computer([ascii::Char; 2]);

type Graph = UnGraphMap<Computer, ()>;

fn main() -> Result<()> {
    let graph = parse_input(INPUT).context("failed to parse input")?;

    println!("part 1: {}", part1(&graph));
    println!("part 2: {}", part2(&graph));

    Ok(())
}

fn part1(graph: &Graph) -> usize {
    cliques3(graph)
        .into_iter()
        .filter(|clique| clique.iter().any(|v| v.0[0].to_u8() == b't'))
        .count()
}

fn part2(graph: &Graph) -> String {
    max_cliques(graph)
        .into_iter()
        .reduce(|a, b| if a.len() < b.len() { b } else { a })
        .map(|clique| {
            let mut clique = clique.into_iter().collect::<Vec<_>>();
            clique.sort_unstable();

            let mut s = String::new();

            for v in clique {
                if !s.is_empty() {
                    s.push(',');
                }
                s.push_str(v.0.as_str());
            }

            s
        })
        .expect("no cliques")
}

fn cliques3(graph: &Graph) -> Vec<[Computer; 3]> {
    let mut cliques = Vec::new();

    let mut a_nodes = graph.nodes();

    while let Some(a) = a_nodes.next() {
        let mut b_nodes = a_nodes.clone();

        while let Some(b) = b_nodes.next() {
            if !graph.contains_edge(a, b) {
                continue;
            }

            for c in b_nodes.clone() {
                if !graph.contains_edge(a, c) || !graph.contains_edge(b, c) {
                    continue;
                }

                cliques.push([a, b, c]);
            }
        }
    }

    cliques
}

fn max_cliques(graph: &Graph) -> Vec<HashSet<Computer>> {
    // Bron–Kerbosch algorithm: https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
    fn bron_kerbosh(
        graph: &Graph,
        cliques: &mut Vec<HashSet<Computer>>,
        r: HashSet<Computer>,
        mut p: HashSet<Computer>,
        mut x: HashSet<Computer>,
    ) {
        let Some(&u) = p.union(&x).next() else {
            cliques.push(r);
            return;
        };

        let nodes = {
            let mut nodes = p.clone();
            for n in graph.neighbors(u) {
                nodes.remove(&n);
            }
            nodes
        };

        for v in nodes {
            let neighbors = graph.neighbors(v).collect();

            bron_kerbosh(
                graph,
                cliques,
                {
                    let mut r = r.clone();
                    r.insert(v);
                    r
                },
                p.intersection(&neighbors).copied().collect(),
                x.intersection(&neighbors).copied().collect(),
            );

            p.remove(&v);
            x.insert(v);
        }
    }

    let mut cliques = Vec::new();

    bron_kerbosh(graph, &mut cliques, HashSet::new(), graph.nodes().collect(), HashSet::new());

    cliques
}

fn parse_input(input: &str) -> Result<Graph> {
    let edges = input.lines().map(|line| {
        let (a, b) = line.split_once('-').context("missing '-' in connection")?;

        let a = a
            .as_ascii()
            .and_then(|a| a.try_into().ok())
            .with_context(|| format!("invalid id: {a}"))?;
        let b = b
            .as_ascii()
            .and_then(|a| a.try_into().ok())
            .with_context(|| format!("invalid id: {b}"))?;

        anyhow::Ok((Computer(a), Computer(b)))
    });

    let mut graph = Graph::new();

    for edge in edges {
        let (a, b) = edge?;

        graph.add_edge(a, b, ());
    }

    Ok(graph)
}

#[cfg(test)]
mod example {
    const EXAMPLE: &str = include_str!("./example");

    #[test]
    fn part1() {
        let graph = super::parse_input(EXAMPLE).unwrap();

        assert_eq!(super::part1(&graph), 7);
    }

    #[test]
    fn part2() {
        let graph = super::parse_input(EXAMPLE).unwrap();

        assert_eq!(super::part2(&graph), "co,de,ka,ta");
    }
}
