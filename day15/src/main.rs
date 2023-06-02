use lazy_static::lazy_static;
use regex::Regex;
use std::arch::x86_64::_mm_insert_si64;
use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"
        )
        .unwrap();
    }
    let mut covered_ranges: Vec<(isize, isize)> = vec![];
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

        let vertical_distance = (sensor.1 - 2000000).abs();
        if vertical_distance > distance {
            continue; // This sensor is too far from the row.
        }
        let horizontal_distance = distance - vertical_distance;
        let mut new_range = (
            sensor.0 - horizontal_distance,
            sensor.0 + horizontal_distance,
        );
        let mut merged_ranges = vec![];
        for range in covered_ranges {
            // This algo is chonky linear wrt the number of sensors
            if range.0 <= new_range.1 || new_range.0 <= range.1 {
                new_range = (min(new_range.0, range.0), max(new_range.1, range.1))
            } else {
                merged_ranges.push(range);
            }
        }
        merged_ranges.push(new_range);
        covered_ranges = merged_ranges;
    }
    let mut covered_positions = 0;
    for range in covered_ranges {
        covered_positions += range.1 - range.0;
    }
    Ok(println!("{covered_positions}"))
}
