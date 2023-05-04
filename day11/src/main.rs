mod monkey;
mod worry_level;

use crate::worry_level::WorryLevel;

use primal::Sieve;
use regex::Regex;
use sort_by_derive::SortBy;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufReader, Read};
use std::iter::successors;

use std::ops::Mul;
use std::rc::Rc;

use monkey::Monkey;
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let f = File::open("test.txt")?;
    let mut reader = BufReader::new(f);
    let mut input = String::new();
    reader.read_to_string(&mut input)?;
    let re = Regex::new(r"Monkey \d:\n").unwrap();
    let monkeys: VecDeque<_> = re
        .split(&input)
        .skip(1) // Skip the first empty string
        .map(|s| s.parse::<Monkey<usize>>().unwrap())
        .collect();

    let mut max_div_by = 0;
    for monkey in &monkeys {
        if monkey.test.div_by > max_div_by {
            max_div_by = monkey.test.div_by;
        }
    }
    let sieve = Rc::new(Sieve::new((max_div_by as f32).sqrt().ceil() as usize));
    let mut monkeys: VecDeque<Monkey<WorryLevel>> = monkeys
        .iter()
        .map(|m| Monkey::from((m.clone(), &sieve)))
        .collect();
    for r in 0..10_000 {
        let mut checked_monkeys: VecDeque<_> = VecDeque::new();
        for i in 0..monkeys.len() {
            let mut monkey = monkeys.pop_front().unwrap();
            monkey.inspect(&mut monkeys, &mut checked_monkeys, i);
            checked_monkeys.push_back(monkey)
        }
        monkeys = checked_monkeys;
        if (r + 1) % 1000 == 0 || r == 20 {
            println!("== After round {r} ==");
            for (i, monkey) in monkeys.iter().enumerate() {
                println!(
                    "Monkey {i} inspected items {} times.",
                    monkey.num_inspections
                );
            }
        }
    }
    println!("== After round 10000 ==");
    for (i, monkey) in monkeys.iter().enumerate() {
        println!(
            "Monkey {i} inspected items {} times.",
            monkey.num_inspections
        );
    }
    let mut sorted_monkeys: Vec<_> = monkeys.iter().cloned().collect();
    sorted_monkeys.sort();
    let monkey_business = sorted_monkeys.pop().unwrap().num_inspections
        * sorted_monkeys.pop().unwrap().num_inspections;
    print!("{monkey_business}");
    Ok(())
}
