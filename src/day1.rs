use rayon::prelude::*;
use itertools::iproduct;

pub fn main() {
    let contents = std::fs::read_to_string("day1").unwrap()
        .lines()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    part1(contents.clone());
    // part2(contents);
}

pub fn part1(contents: Vec<u64>) {

    let product = contents
        .par_iter()
        .flat_map(|i| contents.par_iter().map(move |j| (i, j)))
        .filter(|(i, j)| **i + **j == 2020)
        .map(|(i, j)| {
            i * j
        })
        .collect::<Vec<u64>>();
    println!("{}", product.first().unwrap());
}

pub fn part2(contents: Vec<u64>) {
    let product = iproduct!(contents.clone(), contents.clone(), contents)
        .filter(|(i, j, k)| i + j + k == 2020)
        .map(|(i, j, k)| i * j * k)
        .collect::<Vec<u64>>();

    println!("{}", product.first().unwrap());
}
