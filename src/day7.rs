use petgraph::graphmap::DiGraphMap;
use std::collections::HashMap;
use regex::Regex;

pub fn main() {
    let contents = std::fs::read_to_string("inputs/day7").unwrap().lines().map(ToString::to_string).collect::<Vec<_>>();
    part1(&contents);
    part2(&contents);
}

fn part1(contents: &Vec<String>) {
    let mut graph = DiGraphMap::new();
    // Mapping of bag name to graph id, needed because the node types must impl Copy, and trying to get &str lifetimes functional here would be a nightmare
    let mut names: HashMap<String, usize> = HashMap::new();
    // Rolling count of next node id
    let mut node = 1;

    // depluralify bags to normalize their names
    let re = Regex::new("s$").unwrap();
    for line in contents {
        let mut line = line.split(" contain ");
        let this_node = line.next().unwrap();
        let this_node = *names.entry(re.replace(this_node, "").to_string())
            .or_insert_with(|| {
                let x = node;
                node += 1;
                x
            });
        let connections = line.next().unwrap().split(", ").map(|s| &s[2..])
            .map(|s| {
                // normalize with no periods and in the singular
                re.replace(&s.replace(".", ""), "").to_string()
            })
            // Find existing node id or create one with running id
            .map(|s| *names.entry(s).or_insert_with(|| {
                let x = node;
                node += 1;
                x
            }))
            .collect::<Vec<_>>();

        for con in connections {
            // con -> this_node matters because it's building a DAG.
            // outgoing nodes (a->b) imply that b can contain a, needed to properly traverse the graph for part 1
            graph.add_edge(con, this_node, 1);
        }
    }

    let id = *names.get("shiny gold bag").unwrap();
    println!("{}", traverse_graph_part1(id, &graph, &mut vec![]).len());
}

fn part2(contents: &Vec<String>) {

    let mut graph = DiGraphMap::new();
    // Mapping of bag name to graph id, needed because the node types must impl Copy, and trying to get &str lifetimes functional here would be a nightmare
    let mut names: HashMap<String, usize> = HashMap::new();
    // Rolling count of next node id
    let mut node = 1;

    // depluralify bags to normalize their names
    let re = Regex::new("s$").unwrap();
    for line in contents {
        let mut line = line.split(" contain ");
        let this_node = line.next().unwrap();
        let this_node = *names.entry(re.replace(this_node, "").to_string())
            .or_insert_with(|| {
                let x = node;
                node += 1;
                x
            });
        let connections = line.next().unwrap().split(", ").map(|s| {
            (s[..2].trim().parse::<usize>().unwrap_or(0), &s[2..])
        })
            .map(|(weight, s)| {
                // normalize with no periods and in the singular
                (weight, re.replace(&s.replace(".", ""), "").to_string())
            })
            // Find existing node id or create one with running id
            .map(|(weight, s)| (weight, *names.entry(s).or_insert_with(|| {
                let x = node;
                node += 1;
                x
            })))
            .collect::<Vec<_>>();

        for (weight, con) in connections {
            // con -> this_node matters because it's building a DAG.
            // outgoing nodes (a->b) imply that b can contain a, needed to properly traverse the graph for part 1
            graph.add_edge(this_node, con, weight);
        }
    }

    let id = *names.get("shiny gold bag").unwrap();
    println!("{}", traverse_graph_part2(id, &graph) - 1); // alg with count the shiny gold bag by default
}

/// A fairly naive approach to finding all nodes accessible from a start node
/// Go through all neighbours, and keep track of nodes that have already been seen to stop duplicates
/// keep doing this until there are no more outgoing edges
/// Directed edge here should mean for a->b, a is contained within b
pub fn traverse_graph_part1<'a>(start: usize, graph: &DiGraphMap<usize, u8>, seen: &'a mut Vec<usize>) -> &'a mut Vec<usize> {
    // Starting at `start`, recursively descend every path through every neighbouring node until there aren't any unique outgoing edges to observe anymore
    for neighbor in graph.neighbors(start) {
        // make sure that values aren't duplicated
        if seen.contains(&neighbor) {
            continue;
        }

        // record this value to make sure this node doesn't get double counted, and continue
        seen.push(neighbor);
        traverse_graph_part1(neighbor, graph, seen);
    }

    seen
}
pub fn traverse_graph_part2(start: usize, graph: &DiGraphMap<usize, usize>) -> usize {
    let mut total_weight = 1; // Weight starts at 1 to count the parent bag in this tree

    for (_, neighbor, weight) in graph.edges(start) {
        let total = traverse_graph_part2(neighbor, graph);

        total_weight += total * weight;
    }

    total_weight
}
