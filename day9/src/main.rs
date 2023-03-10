#![feature(portable_simd)]

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::simd::{i32x2, Simd, SimdInt, SimdOrd, SimdPartialOrd};
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let movements = reader
        .lines()
        .map(|line| line.unwrap().parse::<Movement>().unwrap());
    let mut tiles_covered = HashSet::new();
    let mut rope = [i32x2::splat(0); 10];
    tiles_covered.insert(rope[rope.len() - 1]);
    let mut rope_states = vec![];
    for movement in movements {
        let movement_vector = match movement.direction {
            Direction::Up => i32x2::from([0, 1]),
            Direction::Down => i32x2::from([0, -1]),
            Direction::Left => i32x2::from([-1, 0]),
            Direction::Right => i32x2::from([1, 0]),
        };
        for _ in 0..movement.number {
            rope[0] += movement_vector;
            for i in 0..rope.len() - 1 {
                rope[i + 1] = move_knot(&rope[i], &rope[i + 1]);
            }
            tiles_covered.insert(rope[rope.len() - 1]);
            rope_states.push(rope.clone());
        }
    }

    // let extremes = extremes(&tiles_covered);
    // print_states(&rope_states, extremes);
    // print_tiles(&tiles_covered, extremes);

    println!("{}", tiles_covered.len());
    Ok(())
}

fn move_knot(head: &i32x2, tail: &i32x2) -> i32x2 {
    let distance = head - tail;
    if distance.abs().simd_le(i32x2::from([1, 1])).all() {
        return tail.clone();
    }
    let norm_dir = distance / distance.abs().simd_max(i32x2::splat(1));
    return tail + norm_dir;
}

fn print_states<const COUNT: usize>(
    rope_states: &Vec<[i32x2; COUNT]>,
    extremes: (i32, i32, i32, i32),
) {
    let (high, low, left, right) = extremes;

    for state in rope_states {
        for y in (low..high).rev() {
            let mut line = String::new();
            for x in left..right {
                match state.iter().position(|pos| *pos == i32x2::from([x, y])) {
                    Some(idx) => {
                        line.push_str(idx.to_string().as_str());
                    }
                    None => {
                        line.push('.');
                    }
                }
            }
            println!("{line}");
        }
        println!();
    }
}

fn print_tiles(tiles_covered: &HashSet<i32x2>, extremes: (i32, i32, i32, i32)) {
    let (high, low, left, right) = extremes;

    for y in (low..high).rev() {
        let mut line = String::new();
        for x in left..right {
            if tiles_covered.contains(&i32x2::from([x, y])) {
                line.push('#');
            } else {
                line.push('.');
            }
        }
        println!("{line}");
    }
}

fn extremes(tiles_covered: &HashSet<Simd<i32, 2>>) -> (i32, i32, i32, i32) {
    let (mut high, mut low, mut left, mut right) = Default::default();
    for tile in tiles_covered {
        let [x, y] = tile.as_array();
        if *x > right {
            right = *x;
        } else if *x < left {
            left = *x;
        }

        if *y > high {
            high = *y;
        } else if *y < low {
            low = *y;
        }
    }
    (high, low, left, right)
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dont_move_knot() {
        assert_eq!(
            i32x2::from([0, 0]),
            move_knot(&i32x2::from([1, 0]), &i32x2::splat(0))
        );
        assert_eq!(
            i32x2::from([0, 0]),
            move_knot(&i32x2::from([0, 1]), &i32x2::splat(0))
        );
        assert_eq!(
            i32x2::from([0, 0]),
            move_knot(&i32x2::from([-1, 0]), &i32x2::splat(0))
        );
        assert_eq!(
            i32x2::from([0, 0]),
            move_knot(&i32x2::from([0, -1]), &i32x2::splat(0))
        );
        assert_eq!(
            i32x2::from([0, 0]),
            move_knot(&i32x2::from([1, 1]), &i32x2::splat(0))
        );
        assert_eq!(
            i32x2::from([0, 0]),
            move_knot(&i32x2::from([-1, 1]), &i32x2::splat(0))
        );
        assert_eq!(
            i32x2::from([0, 0]),
            move_knot(&i32x2::from([1, -1]), &i32x2::splat(0))
        );
        assert_eq!(
            i32x2::from([0, 0]),
            move_knot(&i32x2::from([-1, -1]), &i32x2::splat(0))
        );
    }

    #[test]
    fn move_knot_() {
        assert_eq!(
            i32x2::from([1, 0]),
            move_knot(&i32x2::from([2, 0]), &i32x2::splat(0))
        );
        assert_eq!(
            i32x2::from([0, 1]),
            move_knot(&i32x2::from([0, 2]), &i32x2::splat(0))
        );
        assert_eq!(
            i32x2::from([-1, 0]),
            move_knot(&i32x2::from([-2, 0]), &i32x2::splat(0))
        );
        assert_eq!(
            i32x2::from([0, -1]),
            move_knot(&i32x2::from([0, -2]), &i32x2::splat(0))
        );
        assert_eq!(
            i32x2::from([1, 1]),
            move_knot(&i32x2::from([2, 2]), &i32x2::splat(0))
        );
        assert_eq!(
            i32x2::from([-1, 1]),
            move_knot(&i32x2::from([-2, 2]), &i32x2::splat(0))
        );
        assert_eq!(
            i32x2::from([1, -1]),
            move_knot(&i32x2::from([2, -2]), &i32x2::splat(0))
        );
        assert_eq!(
            i32x2::from([-1, -1]),
            move_knot(&i32x2::from([-2, -2]), &i32x2::splat(0))
        );
        assert_eq!(
            i32x2::from([1, 1]),
            move_knot(&i32x2::from([2, 1]), &i32x2::splat(0))
        );
        assert_eq!(
            i32x2::from([-1, 1]),
            move_knot(&i32x2::from([-2, 1]), &i32x2::splat(0))
        );
        assert_eq!(
            i32x2::from([1, -1]),
            move_knot(&i32x2::from([2, -1]), &i32x2::splat(0))
        );
        assert_eq!(
            i32x2::from([-1, -1]),
            move_knot(&i32x2::from([-2, -1]), &i32x2::splat(0))
        );
        assert_eq!(
            i32x2::from([1, 1]),
            move_knot(&i32x2::from([1, 2]), &i32x2::splat(0))
        );
        assert_eq!(
            i32x2::from([-1, 1]),
            move_knot(&i32x2::from([-1, 2]), &i32x2::splat(0))
        );
        assert_eq!(
            i32x2::from([1, -1]),
            move_knot(&i32x2::from([1, -2]), &i32x2::splat(0))
        );
        assert_eq!(
            i32x2::from([-1, -1]),
            move_knot(&i32x2::from([-1, -2]), &i32x2::splat(0))
        );
    }
}
