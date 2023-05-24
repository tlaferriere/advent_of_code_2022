use glidesort::sort_in_vec;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufReader, Read};
use std::ptr::write;

#[derive(Parser)]
#[grammar = "lists.pest"]
pub struct ListsParser;

#[derive(PartialEq, Eq, Clone, Debug)]
enum Datagram {
    List(Vec<Datagram>),
    Num(u8),
}

impl Display for Datagram {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Datagram::List(l) => {
                write!(f, "[")?;
                for packet in l {
                    write!(f, "{packet},")?;
                }
                write!(f, "]")?;
                Ok(())
            }
            Datagram::Num(n) => {
                write!(f, "{n}")?;
                Ok(())
            }
        }
    }
}

impl PartialOrd for Datagram {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Datagram::Num(s), Datagram::Num(o)) => s.partial_cmp(o),
            (Datagram::List(s), Datagram::List(o)) => {
                let mut s_iter = s.iter();
                let mut o_iter = o.iter();
                loop {
                    match (s_iter.next(), o_iter.next()) {
                        (Some(l), Some(r)) => match l.partial_cmp(r) {
                            ord @ Some(Ordering::Less | Ordering::Greater) => break ord,
                            None | Some(Ordering::Equal) => continue,
                        },
                        (None, None) => break None,
                        (None, Some(_)) => break Some(Ordering::Less),
                        (Some(_), None) => break Some(Ordering::Greater),
                    }
                }
            }
            (l @ Datagram::List(_), Datagram::Num(r)) => {
                l.partial_cmp(&Datagram::List(vec![Datagram::Num(*r)]))
            }
            (Datagram::Num(l), r @ Datagram::List(_)) => {
                Datagram::List(vec![Datagram::Num(*l)]).partial_cmp(r)
            }
        }
    }
}

impl Ord for Datagram {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            None => Ordering::Equal,
            Some(ord) => ord,
        }
    }
}

impl From<Pair<'_, Rule>> for Datagram {
    fn from(value: Pair<Rule>) -> Self {
        match value.as_rule() {
            Rule::list => Datagram::List(value.into_inner().map(Datagram::from).collect()),
            Rule::num => Datagram::Num(value.as_str().parse().expect("Not a number.")),
            x => {
                panic!("{x:?}")
            }
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

    let mut transmission: Vec<_> = parsed
        .filter(|pair| matches!(pair.as_rule(), Rule::list | Rule::num))
        .map(Datagram::from)
        .collect();
    let div_packet_2 = Datagram::List(vec![Datagram::List(vec![Datagram::Num(2)])]);
    transmission.push(div_packet_2.clone());
    let div_packet_6 = Datagram::List(vec![Datagram::List(vec![Datagram::Num(6)])]);
    transmission.push(div_packet_6.clone());
    sort_in_vec(&mut transmission);
    println!("[");
    for packet in &transmission {
        println!("{packet},");
    }
    println!("]");
    let decoder_key = (transmission.binary_search(&div_packet_2).unwrap() + 1)
        * (transmission.binary_search(&div_packet_6).unwrap() + 1);
    Ok(println!("{decoder_key}"))
}
