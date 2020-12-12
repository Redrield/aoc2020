use std::f64::consts::PI;

pub fn main() {
    let insns = std::fs::read_to_string("inputs/day12").unwrap()
        .lines().map(ToString::to_string).collect::<Vec<_>>();

    part1(&insns);
    part2(&insns);
}

fn part1(insns: &Vec<String>) {
    // coords are +/- in comment
    let mut x = 0i64; // N/S
    let mut y = 0i64; // E/W
    let mut hdg = 90; // North 0, CW+

    for insn in insns {
        let value: i64 = insn[1..].parse().unwrap();
        match &insn[..1] {
            "N" => x += value,
            "S" => x -= value,
            "E" => y += value,
            "W" => y -= value,
            "F" => match hdg {
                0 => x += value,
                180 => x -= value,
                90 => y += value,
                270 => y -= value,
                h => unreachable!(),
            },
            "L" => {
                let new_hdg = hdg - value;
                hdg = new_hdg.rem_euclid(360);
            },
            "R" => {
                let new_hdg = hdg + value;
                hdg = new_hdg.rem_euclid(360);
            }
            _ => unreachable!()
        }
    }

    println!("{}", x.abs() + y.abs());
}

fn part2(insns: &Vec<String>) {
    // coords are +/- in comment
    let mut x = 0i64; // N/S
    let mut wx = 1i64;
    let mut y = 0i64; // E/W
    let mut wy = 10i64;

    for insn in insns {
        let value: i64 = insn[1..].parse().unwrap();
        match &insn[..1] {
            "N" => wx += value,
            "S" => wx -= value,
            "E" => wy += value,
            "W" => wy -= value,
            "F" => {
                x += wx * value;
                y += wy * value;
            },
            "L" => {
                let value = (value as f64) * (PI / 180.0); // convert to float + rad
                // 2x2 rotation widdershins
                let new_wy = wy * value.cos() as i64 - wx * value.sin() as i64;
                let new_wx = wy * value.sin() as i64 + wx * value.cos() as i64;
                wx = new_wx;
                wy = new_wy;
            },
            "R" => {
                let value = (value as f64) * (PI / 180.0); // convert to float + rad
                // 2x2 rotation clockwise
                let new_wy = wy * value.cos() as i64 + wx * value.sin() as i64;
                let new_wx = -wy * value.sin() as i64 + wx * value.cos() as i64;
                wx = new_wx;
                wy = new_wy;
            }
            _ => unreachable!()
        }
    }

    println!("{}", x.abs() + y.abs());
}
