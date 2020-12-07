use rayon::prelude::*;
use itertools::iproduct;
use std::collections::HashMap;

#[derive(Clone)]
struct PasswordCheck {
    letter: char,
    policy_bounds: (u64, u64),
    password: String,
}

fn is_valid1(ck: &PasswordCheck) -> bool {
    let mut map: HashMap<char, u64> = HashMap::new();

    for ch in ck.password.chars() {
        map.entry(ch)
            .and_modify(|freq| *freq += 1)
            .or_insert(1);
    }

    let reqd = map.get(&ck.letter).unwrap_or(&0);
    (ck.policy_bounds.0..=ck.policy_bounds.1).contains(reqd)
}

fn is_valid2(ck: &PasswordCheck) -> bool {
    let mut seen = false;
    let (j, k) = ck.policy_bounds;
    let j = j as usize - 1;
    let k = k as usize - 1;

    for (i, ch) in ck.password.chars().enumerate() {
        if (i == j || i == k) && ch == ck.letter {
            if seen {
                return false;
            } else {
                seen = true;
            }
        }
    }

    seen
}

pub fn main() {
    let contents = std::fs::read_to_string("day2").unwrap()
        .lines()
        .map(|s| {
            let mut ws = s.split_whitespace();
            let mut policy = ws.next().unwrap().split("-");
            let letter = ws.next().unwrap().replace(":", "");
            let password = ws.next().unwrap();
            let policy_low = policy.next().unwrap().parse().unwrap();
            let policy_high = policy.next().unwrap().parse().unwrap();
            PasswordCheck {
                letter: letter.chars().nth(0).unwrap(),
                policy_bounds: (policy_low, policy_high),
                password: password.to_string()
            }
        })
        .collect::<Vec<PasswordCheck>>();

    println!("{}", contents.clone().into_iter().filter(is_valid1).count());
    println!("{}", contents.clone().into_iter().filter(is_valid2).count());
}

