use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let mut elves = BinaryHeap::new();

    let mut total = 0;
    for line in reader.lines().map(|line| line.unwrap().parse::<u32>()) {
        total = match line {
            Ok(calories) => total + calories,
            Err(_) => {
                elves.push(total);
                0
            }
        };
    }
    let top3_calories: u32 = elves.into_sorted_vec().into_iter().rev().take(3).sum();
    println!("{top3_calories}");
    Ok(())
}
