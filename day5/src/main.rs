use std::fs::File;

use std::io::{BufReader, Read};

use lazy_static::lazy_static;
use regex::Regex;

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);
    let mut string = String::new();
    reader.read_to_string(&mut string)?;
    let mut input_parts = string.split("\n\n");
    // Parse starting stacks
    let mut stack_lines = input_parts
        .next()
        .unwrap()
        .lines()
        .filter(|s| !s.is_empty())
        .rev();
    let stack_alignments: Vec<usize> = stack_lines
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .filter(|(_, s)| !s.is_whitespace())
        .map(|(i, _)| i)
        .collect();
    let stack_num = stack_alignments.len();
    let mut stacks = Vec::with_capacity(stack_num);
    for _ in 0..stack_num {
        stacks.push(Vec::new())
    }
    for line in stack_lines {
        for (&alignment, stack) in (&stack_alignments).into_iter().zip(&mut stacks) {
            let c = line.as_bytes()[alignment] as char; // Assuming ascii
            if c.is_alphabetic() {
                stack.push(c)
            }
        }
    }

    // Execute stack ops

    lazy_static! {
        static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    }
    for line in input_parts
        .next()
        .unwrap()
        .lines()
        .filter(|s| !s.is_empty())
    {
        let cap = RE.captures(line).unwrap();
        let num_crates: usize = cap.get(1).unwrap().as_str().parse().unwrap();

        let source: &mut Vec<char> =
            &mut stacks[cap.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1];
        let mut c = source.split_off(source.len() - num_crates);

        let dest: &mut Vec<char> =
            &mut stacks[cap.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1];
        dest.append(&mut c);
    }

    let tops: String = stacks
        .into_iter()
        .map(|stack| *stack.last().unwrap())
        .collect();
    println!("{tops}");
    Ok(())
}
