use crate::worry_level::WorryLevel;
use sort_by_derive::SortBy;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Clone, SortBy, Debug)]
pub struct Monkey<T> {
    #[sort_by]
    pub(crate) num_inspections: usize,
    items: Vec<T>,
    operation: Operation,
    pub(crate) test: Test,
}

impl From<(Monkey<usize>, &Vec<usize>)> for Monkey<WorryLevel> {
    fn from((monkey, divs): (Monkey<usize>, &Vec<usize>)) -> Self {
        Self {
            num_inspections: monkey.num_inspections,
            items: monkey
                .items
                .iter()
                .map(|&it| WorryLevel::from((it, divs)))
                .collect(),

            operation: monkey.operation,
            test: monkey.test,
        }
    }
}

impl Monkey<WorryLevel> {
    pub(crate) fn test(&self, worry_level: WorryLevel) -> usize {
        if worry_level.divisible_by(&self.test.div_by) {
            self.test.true_action
        } else {
            self.test.false_action
        }
    }

    pub(crate) fn inspect(
        &mut self,
        monkeys: &mut VecDeque<Self>,
        checked_monkeys: &mut VecDeque<Self>,
        i: usize,
    ) {
        for item in &self.items {
            let worry_level = self.operation.eval(item.clone());
            let action_index = self.test(worry_level.clone());
            let throwing_to = match action_index.cmp(&i) {
                Ordering::Greater => monkeys.get_mut(action_index - (i + 1)),
                Ordering::Less => checked_monkeys.get_mut(action_index),
                Ordering::Equal => {
                    panic!("Monkey can't throw to itself")
                }
            }
            .unwrap();
            throwing_to.items.push(worry_level);
        }
        self.num_inspections += self.items.len();
        self.items = Default::default(); // Empty the items list
    }
}

#[derive(Copy, Clone, Debug)]
struct Operation {
    op: Op,
    other: OpArg,
}

#[derive(Copy, Clone, Debug)]
enum Op {
    Add,
    Mul,
}

#[derive(Copy, Clone, Debug)]
enum OpArg {
    Const(usize),
    Itself,
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct Test {
    pub(crate) div_by: usize,
    true_action: usize,
    false_action: usize,
}

impl Operation {
    fn eval(self, val: WorryLevel) -> WorryLevel {
        match self.other {
            OpArg::Const(c) => match self.op {
                Op::Add => val + c,
                Op::Mul => val * c,
            },
            OpArg::Itself => match self.op {
                Op::Add => val * 2,
                Op::Mul => val.pow(2),
            },
        }
    }
}

// ============= Parsing ======================
impl<T: FromStr> FromStr for Monkey<T>
where
    <T as FromStr>::Err: Debug,
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let items = lines
            .next()
            .unwrap()
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|s| s.parse::<T>().unwrap())
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

impl FromStr for OpArg {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(OpArg::Itself),
            s => Ok(OpArg::Const(s.parse().unwrap())),
        }
    }
}
