use regex::Regex;
use sort_by_derive::SortBy;
use std::collections::{BinaryHeap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::iter::zip;
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let f = File::open("test.txt")?;
    let mut reader = BufReader::new(f);
    let mut input = String::new();
    reader.read_to_string(&mut input)?;
    let re = Regex::new(r"Monkey \d:\n").unwrap();
    let mut monkeys: VecDeque<_> = re
        .split(&input)
        .skip(1) // Skip the first empty string
        .map(|s| s.parse::<Monkey>().unwrap())
        .collect();
    for _ in 0..20 {
        let mut checked_monkeys: VecDeque<_> = Default::default();
        for i in 0..monkeys.len() {
            let mut monkey = monkeys.pop_front().unwrap();
            for item in &monkey.items {
                let worry_level = monkey.operation.eval(*item);
                let action_index = if worry_level % monkey.test.div_by == 0 {
                    monkey.test.true_action
                } else {
                    monkey.test.false_action
                };
                if action_index > i {
                    monkeys.get_mut(action_index - i)
                } else if action_index < i {
                    checked_monkeys.get_mut(action_index)
                } else {
                    panic!("Monkey can't throw to itself")
                }
                .unwrap()
                .items
                .push(worry_level);
            }
            monkey.num_inspections += monkey.items.len();
            checked_monkeys.push_back(monkey)
        }
        let monkeys = checked_monkeys;
        for (i, monkey) in monkeys.iter().enumerate() {
            println!("Monkey {i}: {:?}", monkey.items);
        }
    }
    let monkey_business = sorted_monkeys.pop().unwrap().num_inspections
        * sorted_monkeys.pop().unwrap().num_inspections;
    print!("{monkey_business}");
    Ok(())
}

#[derive(Clone, SortBy)]
struct Monkey {
    #[sort_by]
    num_inspections: usize,
    items: Vec<u32>,
    operation: Operation,
    test: Test,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let items = lines
            .next()
            .unwrap()
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let operation = lines
            .next()
            .unwrap()
            .strip_prefix("  Operation: new = ")
            .unwrap()
            .parse()
            .unwrap();
        let mut test_str = format!("{}\n", lines.next().unwrap());
        test_str.push_str(&format!("{}\n", lines.next().unwrap()));
        test_str.push_str(&format!("{}\n", lines.next().unwrap()));
        let test = test_str.parse().unwrap();
        Ok(Self {
            num_inspections: 0,
            items,
            operation,
            test,
        })
    }
}

#[derive(Clone)]
struct Operation {
    op: Op,
    other: OpArg,
}

impl Operation {
    fn eval(&self, val: u32) -> u32 {
        let worry_increase = match self.op {
            Op::Add => {
                val + match self.other {
                    OpArg::Const(c) => c,
                    OpArg::Itself => val,
                }
            }
            Op::Mul => {
                val * match self.other {
                    OpArg::Const(c) => c,
                    OpArg::Itself => val,
                }
            }
        };
        (worry_increase as f32 / 3.0).round() as u32
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.strip_prefix("old ").unwrap().split(" ");
        Ok(Self {
            op: parts.next().unwrap().parse()?,
            other: parts.next().unwrap().parse()?,
        })
    }
}

#[derive(Clone)]
enum Op {
    Add,
    Mul,
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Op::Add),
            "*" => Ok(Op::Mul),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
enum OpArg {
    Const(u32),
    Itself,
}

impl FromStr for OpArg {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(OpArg::Itself),
            s => Ok(OpArg::Const(s.parse().unwrap())),
        }
    }
}

#[derive(Clone)]
struct Test {
    div_by: u32,
    true_action: usize,
    false_action: usize,
}

impl FromStr for Test {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        Ok(Self {
            div_by: lines
                .next()
                .unwrap()
                .strip_prefix("  Test: divisible by ")
                .unwrap()
                .parse()
                .unwrap(),
            true_action: lines
                .next()
                .unwrap()
                .strip_prefix("    If true: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap(),
            false_action: lines
                .next()
                .unwrap()
                .strip_prefix("    If false: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap(),
        })
    }
}
