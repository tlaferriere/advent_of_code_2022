use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let mut max_calories = 0;

    let mut total = 0;
    for line in reader.lines().map(|line| line.unwrap().parse::<u32>()) {
        total = match line {
            Ok(calories) => total + calories,
            Err(_) => {
                if total > max_calories {
                    max_calories = total;
                }
                0
            }
        };
    }
    println!("{max_calories}");
    Ok(())
}
