use std::collections::{HashMap, HashSet};
use std::fs::File;

use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let alphabet: HashMap<char, usize> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect();
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let mut total: u32 = 0;
    let mut lines = reader
        .lines()
        .map(|line| line.unwrap().chars().collect::<HashSet<_>>());
    while let (Some(elf1), Some(elf2), Some(elf3)) = (lines.next(), lines.next(), lines.next()) {
        total += *alphabet
            .get(
                elf1.intersection(&elf2)
                    .copied()
                    .collect::<HashSet<_>>()
                    .intersection(&elf3)
                    .next()
                    .unwrap(),
            )
            .unwrap() as u32;
    }
    println!("{total}");
    Ok(())
}
