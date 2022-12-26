use std::fs::File;

use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let mut total: u32 = 0;
    for line in reader.lines() {
        let string = line?;
        let mut nums = string.split(&[',', '-'][..]).map(|s| s.parse().unwrap());
        let [low1, high1, low2, high2]: [u32; 4] = [
            nums.next().unwrap(),
            nums.next().unwrap(),
            nums.next().unwrap(),
            nums.next().unwrap(),
        ];
        if (low1 <= low2 && high1 >= low2) || (low2 <= low1 && high2 >= low1) {
            total += 1
        }
    }
    println!("{total}");
    Ok(())
}
