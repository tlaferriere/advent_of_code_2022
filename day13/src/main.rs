use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Parser)]
#[grammar = "lists.pest"]
pub struct ListsParser;

fn are_in_order(mut left: Pairs<Rule>, mut right: Pairs<Rule>) -> Option<bool> {
    loop {
        match (left.next(), right.next()) {
            (Some(l), Some(r)) => {
                if l != r {
                    match (l.as_rule(), r.as_rule()) {
                        (Rule::list, Rule::list) => {
                            if let order @ Some(_) = are_in_order(l.into_inner(), r.into_inner()) {
                                break order;
                            }
                        }
                        (Rule::list, Rule::num) => {
                            let list = format!("[{}]", r.as_str());
                            let r_list = ListsParser::parse(Rule::list, &list)
                                .unwrap()
                                .next()
                                .unwrap();
                            if let order @ Some(_) =
                                are_in_order(l.into_inner(), r_list.into_inner())
                            {
                                break order;
                            }
                        }
                        (Rule::num, Rule::list) => {
                            let list = format!("[{}]", l.as_str());
                            let l_list = ListsParser::parse(Rule::list, &list)
                                .unwrap()
                                .next()
                                .unwrap();
                            if let order @ Some(_) =
                                are_in_order(l_list.into_inner(), r.into_inner())
                            {
                                break order;
                            }
                        }
                        (Rule::num, Rule::num) => {
                            let (l_num, r_num): (u8, u8) =
                                (l.as_str().parse().unwrap(), r.as_str().parse().unwrap());
                            if l_num != r_num {
                                break Some(l_num < r_num);
                            }
                        }
                        (_, _) => {
                            unreachable!()
                        }
                    }
                }
            }
            (None, None) => break None,
            (l, _) => break Some(l.is_none()),
        }
    }
}

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);
    let mut unparsed = String::new();
    reader.read_to_string(&mut unparsed)?;
    let mut parsed =
        ListsParser::parse(Rule::file, unparsed.as_str()).expect("Unable to parse lists");
    // println!("{parsed:?}");

    // Build up the fs representation
    let mut sum = 0;
    for (i, pair) in parsed.enumerate() {
        match pair.as_rule() {
            Rule::pair => {
                let mut lists: Vec<_> = pair.into_inner().collect();
                assert_eq!(lists.len(), 2);
                let right = lists.pop().unwrap().into_inner();
                let left = lists.pop().unwrap().into_inner();
                if let Some(true) = are_in_order(left, right) {
                    sum += i + 1;
                };
            }
            Rule::EOI => {
                println!("Reached end of input.")
            }
            _ => {
                unreachable!()
            }
        }
    }
    Ok(println!("{sum}"))
}
