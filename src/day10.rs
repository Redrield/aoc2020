use std::collections::HashMap;
use petgraph::graphmap::DiGraphMap;
use petgraph::Direction;

pub fn main() {
    let mut contents = std::fs::read_to_string("inputs/day10").unwrap()
        .lines().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();

    // let mut contents = vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3];
    // let mut contents = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    contents.push(0);
    contents.push(*contents.iter().max().unwrap() + 3);

    contents.sort();
    // part1(&contents[..]);
    part2(&contents[..]);
}

fn part1(contents: &[u64]) {
    let device_rating = contents.last().unwrap() + 3;
    let mut ones = 0;
    let mut threes = 0;

    let mut current_voltage = 0;

    for elem in contents {
        match elem - current_voltage {
            1 => ones += 1,
            3 => threes += 1,
            _ => unreachable!()
        }
        current_voltage = *elem;
    }

    match device_rating - current_voltage {
        1 => ones += 1,
        3 => threes += 1,
        _ => unreachable!()
    }
}

fn count(n: u64, contents: &[u64], map: &mut HashMap<u64, u64>) -> u64 {
    // look for potential paths converging at n
    let min = (n as i64 - 3).max(0) as u64;
    (min..n).filter(|i| contents.contains(i))
        .map(|i| {
            // lookup, if nonexistent then calculate and cache
            if let Some(j) = map.get(&i) {
                *j
            } else {
                let c = count(i, contents, map);
                map.insert(i, c);
                c
            }
        })
        // sum the number of paths of all the potential parent nodes
        .sum()
}

fn part2(contents: &[u64]) {
    let mut assoc = HashMap::new();
    // add base case
    assoc.insert(*contents.first().unwrap(), 1);
    println!("{}", count(*contents.last().unwrap(), contents, &mut assoc));
}
