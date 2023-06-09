use lazy_static::lazy_static;
use regex::Regex;

use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"
        )
        .unwrap();
    }
    const GRID_MIN: isize = 0;
    const GRID_MAX: isize = 4_000_000;

    let mut covered_ranges: HashMap<isize, Vec<(isize, isize)>> =
        HashMap::with_capacity((GRID_MAX + 1) as usize);
    for i in GRID_MIN..=GRID_MAX {
        covered_ranges.insert(i, vec![]);
    }

    for line in reader
        .lines()
        .filter_map(|line| line.ok().filter(|line| !line.is_empty()))
    {
        let caps = RE.captures(&line).unwrap();
        let sensor = (
            caps.get(1).unwrap().as_str().parse::<isize>().unwrap(),
            caps.get(2).unwrap().as_str().parse::<isize>().unwrap(),
        );
        let beacon = (
            caps.get(3).unwrap().as_str().parse::<isize>().unwrap(),
            caps.get(4).unwrap().as_str().parse::<isize>().unwrap(),
        );
        let distance = (beacon.0 - sensor.0).abs() + (beacon.1 - sensor.1).abs();

        let mut covered_rows = Vec::new();

        for (row, ranges) in covered_ranges.iter_mut() {
            let vertical_distance = (sensor.1 - *row).abs();
            if vertical_distance > distance {
                continue; // This sensor is too far from the row.
            }
            let horizontal_distance = distance - vertical_distance;
            let mut new_range = (
                sensor.0 - horizontal_distance,
                sensor.0 + horizontal_distance,
            );
            let mut merged_ranges = vec![];
            for range in &*ranges {
                // This algo is chonky linear wrt the number of sensors
                if (range.0 <= new_range.1 + 1 && range.1 >= new_range.0)
                    || (new_range.0 <= range.1 + 1 && new_range.1 >= range.0)
                {
                    new_range = (min(new_range.0, range.0), max(new_range.1, range.1))
                } else {
                    merged_ranges.push(*range);
                }
            }
            merged_ranges.push(new_range);
            let range = merged_ranges.first().unwrap();
            if range.0 <= GRID_MIN && range.1 > GRID_MAX {
                // Mark this row to be removed because it is completely covered.
                covered_rows.push(*row)
            }
            *ranges = merged_ranges;
        }
        for row in covered_rows {
            covered_ranges.remove(&row);
        }
    }
    assert_eq!(covered_ranges.len(), 1, "Not enough rows eliminated");
    let Some((row, ranges)) = covered_ranges.into_iter().next() else {unreachable!()};
    assert!(ranges.len() <= 2 && !ranges.is_empty());
    let mut iter = ranges.into_iter();
    let x = match (iter.next(), iter.next()) {
        (Some(r1), Some(r2)) => {
            if r1.1 + 2 == r2.0 {
                r1.1 + 1
            } else if r2.1 + 2 == r1.0 {
                r2.1 + 1
            } else {
                panic!("Ranges do not leave only one position uncovered.")
            }
        }

        (Some(r), None) => {
            if GRID_MIN - 1 == r.0 {
                GRID_MIN
            } else if r.1 == GRID_MAX - 1 {
                GRID_MAX
            } else {
                panic!("Ranges do not leave only one position uncovered.")
            }
        }
        _ => {
            unreachable!()
        }
    };
    let freq = x * 4000000 + row;
    Ok(println!("{freq}"))
}
