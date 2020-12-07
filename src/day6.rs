use itertools::Itertools;

pub fn main() {
    let contents = std::fs::read_to_string("inputs/day6").unwrap().lines().map(ToString::to_string).collect::<Vec<String>>();

    part1(&contents);
    part2(&contents);
}

fn part1(it: &Vec<String>) {
    let c: usize = it.iter().group_by(|l| l.as_str() != "")
        .into_iter()
        .filter(|(k, _)| *k)
        .flat_map(|(_, s)| s.flat_map(|s| s.chars()).unique())
        .count();
    println!("{}", c);
}

fn part2(it: &Vec<String>) {

    let c: usize = it.iter().group_by(|l| l.as_str() != "")
        .into_iter()
        .filter(|(k, _)| *k)
        .map(|(_, vs)| {
            // Needed to ensure full iterator each loop, using `vs` directly breaks it
            let v = vs.collect::<Vec<_>>();
            ('a'..='z').filter(move |ch| v.iter().all(|s| s.contains(*ch))).count()
        })
        .sum();
    println!("{}", c);
}