use pathfinding::num_traits::abs;
use pathfinding::prelude::astar;
use std::collections::HashMap;
use std::fs::{read, File};
use std::hash::Hash;
use std::io::{BufRead, BufReader, Read};
use std::rc::Rc;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos((i32, i32), Rc<Vec<Vec<usize>>>);

impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        (abs(self.0 .0 - other.0 .0) + abs(self.0 .1 - other.0 .1)) as u32
    }

    fn successors(&self) -> Vec<(Pos, u32)> {
        let Pos((x, y), topo) = self;
        let max_height = topo
            .get(*y as usize)
            .expect("Moved out of y bounds")
            .get(*x as usize)
            .expect("Moved out of x bounds");
        vec![
            Pos((x + 1, *y), topo.clone()),
            Pos((x - 1, *y), topo.clone()),
            Pos((*x, y + 1), topo.clone()),
            Pos((*x, y - 1), topo.clone()),
        ]
        .into_iter()
        .filter(|Pos((x, y), topo)| {
            let Some(row) = topo.get(*y as usize) else { return false; };
            let Some(z) = row.get(*x as usize) else { return false; };
            *z <= *max_height + 1
        })
        .map(|p| (p, 1))
        .collect()
    }
}

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let heights: HashMap<char, usize> = "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .enumerate()
        .map(|(i, c)| (c, i))
        .collect();
    let mut start: Option<(i32, i32)> = None;
    let mut end: Option<(i32, i32)> = None;
    let mut topo = vec![];
    for (j, line) in reader.lines().enumerate() {
        let line = line?;
        let mut row = Vec::with_capacity(line.len());
        for (i, c) in line.chars().enumerate() {
            row.push(match heights.get(&c) {
                None => match c {
                    'S' => {
                        start = Some((i as i32, j as i32));
                        0
                    }
                    'E' => {
                        end = Some((i as i32, j as i32));
                        heights.len() - 1
                    }
                    _ => {
                        unreachable!()
                    }
                },
                Some(h) => *h,
            });
        }
        topo.push(row);
    }

    let topo = Rc::new(topo);
    let start = Pos(start.expect("Start position not found"), topo.clone());
    let end = Pos(end.expect("Start position not found"), topo.clone());

    let result = astar(
        &start,
        |p| p.successors(),
        |p| p.distance(&end),
        |p| p.0 == end.0,
    );
    // .expect("Path could not be found");
    println!("{}", result.unwrap().1);
    Ok(())
}
