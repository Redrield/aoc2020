
pub fn main() {
    let contents = std::fs::read_to_string("inputs/day3").unwrap().lines().map(ToString::to_string).collect::<Vec<String>>();
    part1(&contents);
    part2(&contents);
}

fn part1(contents: &Vec<String>) {
    let mut x = 0;
    let mut tree = 0u64;

    for line in contents {
        let ch = line.chars().nth(x % line.len()).unwrap();
        if ch == '#' {
            tree += 1;
        }
        x += 3
    }

    println!("{}", tree);
}

fn part2(contents: &Vec<String>) {
    let mut x = 0;

    let mut tot = 1u64;

    for slope in vec![1usize, 3, 5, 7] {
        let mut tree = 0u64;

        for line in contents {
            let ch = line.chars().nth(x % line.len()).unwrap();
            if ch == '#' {
                tree += 1;
            }
            x += slope
        }

        x = 0;
        tot *= tree;
    }

    let mut tree = 0u64;
    x = 0;

    for line in contents.iter().step_by(2) {
        let ch = line.chars().nth(x % line.len()).unwrap();
        if ch == '#' {
            tree += 1;
        }
        x += 1;
    }

    tot *= tree;

    println!("{}", tot);

}