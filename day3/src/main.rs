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
    for line in reader.lines() {
        let string = line?;
        let len = string.len();
        let (first, second) = string.split_at(len / 2);
        let second_charset = second.chars().collect();
        total += first
            .chars()
            .collect::<HashSet<_>>()
            .intersection(&second_charset)
            .map(|c| *alphabet.get(c).unwrap() as u32)
            .sum::<u32>();
    }
    println!("{total}");
    Ok(())
}
