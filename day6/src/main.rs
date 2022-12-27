use std::collections::HashSet;
use std::fs::File;

use std::io::{BufRead, BufReader, Read};

fn main() -> std::io::Result<()> {
    let mut f = File::open("input.txt")?;
    let mut buf = vec![];
    f.read_to_end(&mut buf)?;
    for (step, window) in buf.as_slice().windows(4).enumerate() {
        let mut char_set = HashSet::with_capacity(4);
        for c in window {
            if !char_set.insert(*c) {
                break;
            }
        }
        if char_set.len() == 4 {
            let pos = step + 4;
            println!("{pos}");
            break;
        }
    }
    Ok(())
}
