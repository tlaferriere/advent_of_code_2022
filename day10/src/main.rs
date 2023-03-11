use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let mut lines = reader.lines();
    let mut registers = Default::default();
    let mut alu_state = AluState::Ready(registers);
    let mut signal_strength_sum = 0;
    for i in 1..=220 {
        registers = match alu_state {
            AluState::Ready(result) => {
                let Some(line) = lines.next() else { break; };
                let instruction: Instruction = line.unwrap().parse().unwrap();
                match instruction {
                    Instruction::Noop => {
                        alu_state = AluState::Ready(result);
                    }
                    Instruction::Add(n) => {
                        alu_state = AluState::Busy {
                            cycles_left: 1,
                            result: Registers { x: result.x + n },
                        }
                    }
                }
                result
            }
            AluState::Busy {
                cycles_left: 1,
                result,
            } => {
                alu_state = AluState::Ready(result);
                registers
            }
            AluState::Busy {
                cycles_left,
                result,
            } => {
                alu_state = AluState::Busy {
                    cycles_left: cycles_left - 1,
                    result,
                };
                registers
            }
        };
        let signal_strength = i * registers.x;
        if (i - 20) % 40 == 0 {
            signal_strength_sum += signal_strength;
        }
        println!("cycle = {i}, alu = {alu_state:?}, signal_strength = {signal_strength}, signal_strength_sum = {signal_strength_sum}")
    }
    println!("{signal_strength_sum}");
    Ok(())
}

#[derive(Copy, Clone, Debug)]
struct Registers {
    x: i32,
}

impl Default for Registers {
    fn default() -> Self {
        Self { x: 1 }
    }
}

#[derive(Debug)]
enum AluState {
    Ready(Registers),
    Busy { cycles_left: u32, result: Registers },
}

enum Instruction {
    Noop,
    Add(i32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let "noop" = s {
            return Ok(Instruction::Noop);
        }
        let Some(("addx", num)) = s.split_once(' ') else { return Err(()) };
        Ok(Instruction::Add(num.parse().unwrap()))
    }
}
