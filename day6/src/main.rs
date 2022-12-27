use std::collections::HashSet;
use std::fs::File;

use std::io::{BufRead, BufReader, Read};

const MARKER_LEN: usize = 14;

fn main() -> std::io::Result<()> {
    let mut f = File::open("input.txt")?;
    let mut buf = vec![];
    f.read_to_end(&mut buf)?;
    for (step, window) in buf.as_slice().windows(MARKER_LEN).enumerate() {
        let mut char_set = HashSet::with_capacity(MARKER_LEN);
        for c in window {
            if !char_set.insert(*c) {
                break;
            }
        }
        if char_set.len() == MARKER_LEN {
            let pos = step + MARKER_LEN;
            println!("{pos}");
            break;
        }
    }
    Ok(())
}
