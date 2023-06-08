mod monkey;
mod worry_level;
use crate::worry_level::WorryLevel;
use monkey::Monkey;
use regex::Regex;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);
    let mut input = String::new();
    reader.read_to_string(&mut input)?;
    let re = Regex::new(r"Monkey \d:\n").unwrap();
    let monkeys: VecDeque<_> = re
        .split(&input)
        .skip(1) // Skip the first empty string
        .map(|s| s.parse::<Monkey<usize>>().unwrap())
        .collect();

    let divs: Vec<_> = monkeys.iter().map(|monkey| monkey.test.div_by).collect();

    let mut max_div_by = 0;
    for monkey in &monkeys {
        if monkey.test.div_by > max_div_by {
            max_div_by = monkey.test.div_by;
        }
    }
    let mut monkeys: VecDeque<Monkey<WorryLevel>> = monkeys
        .into_iter()
        .map(|m| Monkey::from((m, &divs)))
        .collect();
    for r in 1..=10_000 {
        let mut checked_monkeys: VecDeque<_> = VecDeque::new();
        for i in 0..monkeys.len() {
            let mut monkey = monkeys.pop_front().unwrap();
            monkey.inspect(&mut monkeys, &mut checked_monkeys, i);
            checked_monkeys.push_back(monkey)
        }
        monkeys = checked_monkeys;
        if r % 1000 == 0 || r == 20 || r == 1 {
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
