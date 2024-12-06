use std::cmp::Ordering;

use anyhow::{Context, Result};
use petgraph::csr::Csr;
use petgraph::Directed;

const INPUT: &str = include_str!("./input");

type RuleGraph = Csr<(), (), Directed, u32>;

struct Rule {
    before: u32,
    after: u32,
}

struct Update {
    pages: Vec<u32>,
}

impl Update {
    fn is_correctly_ordered(&self, graph: &RuleGraph) -> bool {
        for (i, page) in self.pages.iter().copied().enumerate() {
            for check in self.pages[..i].iter().copied() {
                if graph.contains_edge(page, check) {
                    return false;
                }
            }
        }

        true
    }
}

fn main() -> Result<()> {
    let (rules, updates) = parse_input().context("failed to parse input")?;

    let mut edges = rules.into_iter().map(|rule| (rule.before, rule.after)).collect::<Vec<_>>();
    edges.sort();
    let graph = RuleGraph::from_sorted_edges(&edges).unwrap();

    part1(&graph, &updates);
    part2(&graph, &updates);

    Ok(())
}

fn part1(graph: &RuleGraph, updates: &[Update]) {
    let sum = updates
        .iter()
        .filter(|update| update.is_correctly_ordered(graph))
        .map(|update| update.pages[update.pages.len() / 2])
        .sum::<u32>();

    println!("part1: {sum}");
}

fn part2(graph: &RuleGraph, updates: &[Update]) {
    let sum = updates
        .iter()
        .filter(|update| !update.is_correctly_ordered(graph))
        .map(|update| {
            let mut pages = update.pages.clone();

            pages.sort_by(|a, b| {
                if a == b {
                    return Ordering::Equal;
                } else if graph.contains_edge(*a, *b) {
                    return Ordering::Less;
                } else if graph.contains_edge(*b, *a) {
                    return Ordering::Greater;
                }

                let forward =
                    petgraph::algo::k_shortest_path(graph, *a, Some(*b), 1, |_| 1u32).remove(b);
                let backward =
                    petgraph::algo::k_shortest_path(graph, *b, Some(*a), 1, |_| 1u32).remove(b);

                match (forward, backward) {
                    (Some(forward), Some(backward)) if forward <= backward => Ordering::Less,
                    (Some(_), Some(_)) => Ordering::Greater,
                    (Some(_), None) => Ordering::Less,
                    (None, Some(_)) => Ordering::Greater,
                    (None, None) => panic!("no path between {a} an {b}"),
                }
            });

            pages
        })
        .map(|pages| pages[pages.len() / 2])
        .sum::<u32>();

    println!("part2: {sum}");
}

fn parse_input() -> Result<(Vec<Rule>, Vec<Update>)> {
    let mut lines = INPUT.lines().enumerate();

    let mut rules = Vec::new();
    let mut updates = Vec::new();

    for (i, line) in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        rules.push(parse_rule(line).with_context(|| format!("invalid rule on line {i}"))?);
    }

    for (i, line) in lines {
        if line.is_empty() {
            break;
        }

        updates.push(parse_update(line).with_context(|| format!("invalid update on line {i}"))?);
    }

    Ok((rules, updates))
}

fn parse_rule(line: &str) -> Result<Rule> {
    let (before, after) = line.split_once('|').context("missing '|' in rule")?;

    Ok(Rule { before: parse_page_number(before)?, after: parse_page_number(after)? })
}

fn parse_update(line: &str) -> Result<Update> {
    line.split(',').map(parse_page_number).collect::<Result<Vec<_>>>().map(|pages| Update { pages })
}

fn parse_page_number(page: &str) -> Result<u32> {
    page.parse::<u32>().with_context(|| format!("invalid page number: '{page}'"))
}
