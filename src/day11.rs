use std::fmt::{Display, Formatter, self};
use std::panic;

const WIDTH: usize = 96;
const HEIGHT: usize = 90;

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Occupied,
    Floor
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Cell::Empty => f.write_str("L"),
            Cell::Occupied => f.write_str("#"),
            Cell::Floor => f.write_str("."),
        }
    }
}

pub fn main() {
    let grid = std::fs::read_to_string("inputs/day11").unwrap()
        // start
        .lines()
        .map(|s| s.chars().map(|c| match c {
            'L' => Cell::Empty,
            '#' => Cell::Occupied,
            '.' => Cell::Floor,
            _ => unreachable!()
        }).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    go(grid);
}

fn go(start: Vec<Vec<Cell>>) {
    let mut current = start;

    loop {
        let new = step(&current);
        if new  == current {
            let occupied = new.into_iter().flatten().filter(|c| *c == Cell::Occupied).count();
            println!("{}", occupied);
            break;
        }
        current = new;
    }
}

pub fn step(grid: &Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {

    let mut new = vec![vec![Cell::Empty; grid.first().unwrap().len()]; grid.len()];

    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            let mut filled = 0;

            for dcol in -1isize..=1 {
                for drow in -1isize..=1 {
                    if dcol == 0 && drow == 0 {
                        continue;
                    }

                    for k in 1.. {
                        match grid.get((i as isize + (dcol * k)) as usize).and_then(|row| row.get((j as isize + (drow * k)) as usize)) {
                            Some(sibling) => {
                                match sibling {
                                    Cell::Empty => {
                                        break;
                                    }
                                    Cell::Occupied => {
                                        filled += 1;
                                        break;
                                    },
                                    Cell::Floor => {}
                                }
                                if *sibling == Cell::Occupied {
                                    filled += 1;
                                }
                            }
                            _ => break,
                        }
                    }
                }
            }

            if *cell == Cell::Empty && filled == 0 {
                new[i][j] = Cell::Occupied;
            } else if *cell == Cell::Occupied && filled >= 5 {
                new[i][j] = Cell::Empty;
            } else {
                new[i][j] = *cell;
            }
        }
    }
    new
}
