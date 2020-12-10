const INVALID: u64 = 375054920; // answer from pt1

pub fn main() {
    let then = std::time::Instant::now();
    let contents = std::fs::read_to_string("inputs/day9").unwrap()
        .lines().map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    part1(&contents[..]);
    part2(&contents[..]);
    let now = std::time::Instant::now();
    println!("{:?}", now - then);
}

fn valid(window: &[u64]) -> bool {
    let preamble = &window[..window.len() - 1];
    let last = window[window.len() - 1];
    preamble.iter()
        // group like day 1
        .flat_map(move |i| preamble.iter().map(move |j| (*i, *j)))
        // unique pair
        .filter(|(i, j)| *i != *j)
        // do they sum properly
        .find(|(i, j)| i + j == last)
        // does it exist
        .is_some()
}

fn part1(contents: &[u64]) {
    let wnd = contents.windows(26)
        .filter(|wnd| !valid(*wnd))
        .next().unwrap();

    println!("{}", wnd[wnd.len() - 1]);
}

pub fn part2(contents: &[u64]) {
    // every window size at least 2
    for n in 2..contents.len() {
        // if the window summed gives INVALID, find min max and add them
        if let Some(wnd) = contents.windows(n).find(|w| w.iter().sum::<u64>() == INVALID) {
            let min = wnd.iter().min().unwrap();
            let max = wnd.iter().max().unwrap();
            println!("{}", min + max);
            break;
        }
    }
}
