use pest::Parser;
use pest_derive::Parser;
use std::arch::x86_64::_mm256_set_epi16;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::{repeat, zip};
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let rocks: Vec<Rock> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.parse().expect("Unable to parse rock")
        })
        .collect();

    let (min_x, max_x, min_y, max_y) = rocks.iter().fold(
        (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
        |(min_x, max_x, min_y, max_y), Rock(corners)| {
            corners.iter().fold(
                (min_x, max_x, min_y, max_y),
                |(min_x, max_x, min_y, max_y), (x, y)| {
                    (
                        min(*x, min_x),
                        max(*x, max_x),
                        min(*y, min_y),
                        max(*y, max_y),
                    )
                },
            )
        },
    );

    const SAND_X: isize = 500;
    const SAND_Y: isize = 0;
    let mut map = Map::new(
        min(SAND_X, min_x),
        max(SAND_X, max_x),
        min(SAND_Y, min_y),
        max(SAND_Y, max_y),
    );

    // Put the rocks on the map.
    for rock in rocks {
        let mut corners = rock.0.iter();
        let (mut start, mut end) = (corners.next().unwrap(), corners.next().unwrap());
        while let (Some(s), Some(e)) = {
            if start.0 != end.0 {
                // Horizontal line
                assert_eq!(start.1, end.1);
                for (i, j) in zip(min(start.0, end.0)..=max(start.0, end.0), repeat(start.1)) {
                    *map.get_mut(i, j) = Point::Rock;
                }
            } else {
                // Vertical line
                assert_ne!(start.1, end.1);
                for (i, j) in zip(repeat(start.0), min(start.1, end.1)..=max(start.1, end.1)) {
                    *map.get_mut(i, j) = Point::Rock;
                }
            }
            (Some(end), corners.next())
        } {
            (start, end) = (s, e);
        }
    }

    // Simulate sand
    let mut rested_sand = 0;
    'sim: loop {
        let (mut x, mut y) = (SAND_X, SAND_Y);
        'grain: loop {
            for (x_fall, y_fall) in [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)] {
                let falls_to = map.get(x_fall, y_fall);
                if let Point::Air = falls_to {
                    (x, y) = (x_fall, y_fall);
                    continue 'grain; // Grain of sand falls further
                }
            }
            // Grain of sand has come to rest
            *map.get_mut(x, y) = Point::Sand;
            rested_sand += 1;
            if (x, y) == (SAND_X, SAND_Y) {
                break 'sim; // Sand source is blocked
            }
            break 'grain;
        }
        // dbg!(&map);
    }

    Ok(println!("{rested_sand}"))
}

#[derive(Parser)]
#[grammar = "rock.pest"]
pub struct RockParser;

struct Rock(Vec<(isize, isize)>);

impl FromStr for Rock {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            RockParser::parse(Rule::rock, s)?
                .next()
                .unwrap()
                .into_inner()
                .map(|corner| -> anyhow::Result<_> {
                    assert!(matches!(corner.as_rule(), Rule::corner));
                    let mut coords = corner.into_inner();
                    let x = coords.next().unwrap().as_str();
                    let y = coords.next().unwrap().as_str();
                    Ok((x.parse()?, y.parse()?))
                })
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

#[derive(Copy, Clone)]
enum Point {
    Air,
    Sand,
    Rock,
}

struct Map {
    x_offset: isize,
    y_offset: isize,
    buf: Vec<Vec<Point>>,
    outside: HashMap<(isize, isize), Point>,
}

impl Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for line in &self.buf {
            for point in line {
                let point_fmt = match point {
                    Point::Air => ".",
                    Point::Sand => "o",
                    Point::Rock => "#",
                };
                write!(f, "{point_fmt}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<'a> Map {
    fn new(x_min: isize, x_max: isize, y_min: isize, y_max: isize) -> Self {
        let y_cap = (y_max + 1 - y_min) as usize;
        let mut buf = Vec::with_capacity(y_cap);
        for _ in 0..y_cap {
            buf.push(vec![Point::Air; (x_max + 1 - x_min) as usize])
        }
        Self {
            x_offset: x_min,
            y_offset: y_min,
            buf,
            outside: HashMap::new(),
        }
    }

    fn get(&'a self, x: isize, y: isize) -> &'a Point {
        let len = self.buf.len() as isize;
        if let Some(point) = self
            .buf
            .get((y - self.y_offset) as usize)
            .and_then(|row| row.get((x - self.x_offset) as usize))
            .or_else(|| self.outside.get(&(x, y)))
        {
            point
        } else if y > self.y_offset + len {
            &Point::Rock
        } else {
            &Point::Air
        }
    }

    fn get_mut(&'a mut self, x: isize, y: isize) -> &'a mut Point {
        let len = self.buf.len() as isize;
        if let Some(point) = self
            .buf
            .get_mut((y - self.y_offset) as usize)
            .and_then(|row| row.get_mut((x - self.x_offset) as usize))
        {
            point
        } else {
            self.outside
                .entry((x, y))
                .or_insert(if y > self.y_offset + len {
                    Point::Rock
                } else {
                    Point::Air
                })
        }
    }
}
