use pathfinding::num_traits::abs;
use pathfinding::prelude::astar;
use std::collections::HashMap;
use std::fs::{read, File};
use std::hash::Hash;
use std::io::{BufRead, BufReader, Read};
use std::rc::Rc;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    pub(crate) fn success(&self, topo: &Vec<Vec<usize>>) -> bool {
        *topo
            .get(self.1 as usize)
            .expect("Moved out of y bounds")
            .get(self.0 as usize)
            .expect("Moved out of x bounds")
            == 0
    }
    pub(crate) fn heuristic(&self, topo: &Vec<Vec<usize>>, lows: &Vec<Pos>) -> u32 {
        let mut closest_dist = topo.len() as u32 * 2;
        for low in lows {
            let dist = self.distance(low);
            if dist < closest_dist {
                closest_dist = dist;
            }
        }
        closest_dist
    }
    fn distance(&self, other: &Pos) -> u32 {
        (abs(self.0 - other.0) + abs(self.1 - other.1)) as u32
    }

    fn successors(&self, topo: &Vec<Vec<usize>>) -> Vec<(Pos, u32)> {
        let Pos(x, y) = self;
        let max_height = topo
            .get(*y as usize)
            .expect("Moved out of y bounds")
            .get(*x as usize)
            .expect("Moved out of x bounds");
        vec![
            Pos(x + 1, *y),
            Pos(x - 1, *y),
            Pos(*x, y + 1),
            Pos(*x, y - 1),
        ]
        .into_iter()
        .filter(|Pos(x, y)| {
            let Some(row) = topo.get(*y as usize) else { return false; };
            let Some(z) = row.get(*x as usize) else { return false; };
            *max_height - 1 <= *z
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
    let mut lows = vec![];
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
                Some(0) => {
                    lows.push(Pos(i as i32, j as i32));
                    0
                }
                Some(h) => *h,
            });
        }
        topo.push(row);
    }

    let (xs, ys) = start.expect("Start position not found");
    let start = Pos(xs, ys);
    let (xe, ye) = end.expect("Start position not found");
    let end = Pos(xe, ye);

    let result = astar(
        &end,
        |p| p.successors(&topo),
        |p| p.heuristic(&topo, &lows),
        |p| p.success(&topo),
    );
    // .expect("Path could not be found");
    println!("{}", result.unwrap().1);
    Ok(())
}
