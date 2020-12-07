use itertools::Itertools;

pub fn main() {
    let contents = std::fs::read_to_string("day5").unwrap().lines().map(ToString::to_string).collect::<Vec<String>>();

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &Vec<String>) {
    let max = contents.iter()
        .map(|s| (s[..7].to_string(), s[7..].to_string()))
        .map(|(s1, s2)| (search(0, 128, s1.chars()), search(0, 8, s2.chars())))
        .map(|(row, col)| row * 8 + col)
        .max().unwrap();
    println!("{}", max);
}

fn part2(contents: &Vec<String>) {
    contents.iter()
        .map(|s| (s[..7].to_string(), s[7..].to_string()))
        .map(|(s1, s2)| (search(0, 128, s1.chars()), search(0, 8, s2.chars())))
        .map(|(row, col)| row * 8 + col)
        .sorted()
        .tuple_windows::<(_, _)>()
        .filter(|(a, b)| *b != *a + 1)
        .for_each(|t| println!("{:?}", t));
}

pub fn search(low: u32, high: u32, mut it: impl Iterator<Item=char>) -> u32 {
    it.fold(low..high, |acc, ch| {
        let mid = (acc.start + acc.end) / 2;
        if ch == 'F' || ch == 'L' {
            acc.start..mid
        } else {
            mid..acc.end
        }
    }).start
}
