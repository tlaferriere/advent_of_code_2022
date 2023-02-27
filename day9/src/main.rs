#![feature(portable_simd)]

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::simd::{i32x2, u32x2, SimdInt, SimdPartialOrd};
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let movements = reader
        .lines()
        .map(|line| line.unwrap().parse::<Movement>().unwrap());
    let mut tiles_covered = HashSet::new();
    let mut head_pos = i32x2::splat(0);
    let mut tail_pos = i32x2::splat(0);
    tiles_covered.insert(tail_pos);
    for movement in movements {
        let movement_vector = match movement.direction {
            Direction::Up => i32x2::from([0, 1]),
            Direction::Down => i32x2::from([0, -1]),
            Direction::Left => i32x2::from([-1, 0]),
            Direction::Right => i32x2::from([1, 0]),
        };
        for _ in 0..movement.number {
            let new_head_pos = head_pos + movement_vector;
            if (new_head_pos - tail_pos)
                .abs()
                .simd_gt(i32x2::from([1, 1]))
                .any()
            {
                tail_pos = head_pos;
                tiles_covered.insert(tail_pos);
            }
            head_pos = new_head_pos;
        }
    }
    println!("{}", tiles_covered.len());
    Ok(())
}

struct Movement {
    direction: Direction,
    number: i32,
}

impl FromStr for Movement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, num) = s.split_once(" ").unwrap();
        Ok(Movement {
            direction: dir.parse().unwrap(),
            number: num.parse().unwrap(),
        })
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(()),
        }
    }
}
