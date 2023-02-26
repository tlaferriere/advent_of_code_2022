#![feature(portable_simd)]

use array2d::Array2D;

use std::fs::File;
use std::io::{BufRead, BufReader};

use std::simd::{u32x4, Mask, SimdPartialEq, SimdPartialOrd, SimdUint};

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let forest_vecs: Vec<_> = reader
        .lines()
        .map(|s| {
            s.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let forest = Array2D::from_rows(&forest_vecs).unwrap();
    let rows = forest.as_rows();
    let columns = forest.as_columns();

    let mut max_scenic_score = 0;
    let edges = u32x4::from([0, 0, forest.row_len() as u32, forest.column_len() as u32]);
    for ((i, j), h) in forest.enumerate_row_major() {
        if u32x4::from([i as u32, j as u32, i as u32, j as u32])
            .simd_eq(edges)
            .any()
        {
            continue;
        }
        let (up, down) = columns.get(j).unwrap().split_at(i);
        let (left, right) = rows.get(i).unwrap().split_at(j);
        let lens = [up.len(), down.len(), left.len(), right.len()];
        let (mut up, mut down, mut left, mut right) = (
            up.iter().rev(),
            down.iter().skip(1),
            left.iter().rev(),
            right.iter().skip(1),
        );
        let height = u32x4::splat(*h);
        let mut views_blocked = Mask::splat(false);
        let mut views = u32x4::splat(0);
        for _ in 0..itertools::max(lens).unwrap() {
            let trees_seen = u32x4::from([
                match up.next() {
                    None => {
                        views_blocked.set(0, true);
                        0
                    }
                    Some(t) => *t,
                },
                match down.next() {
                    None => {
                        views_blocked.set(1, true);
                        0
                    }
                    Some(t) => *t,
                },
                match left.next() {
                    None => {
                        views_blocked.set(2, true);
                        0
                    }
                    Some(t) => *t,
                },
                match right.next() {
                    None => {
                        views_blocked.set(3, true);
                        0
                    }
                    Some(t) => *t,
                },
            ]);
            let inc = views_blocked.select(u32x4::splat(0), u32x4::splat(1));
            views += inc;

            let higher_trees = trees_seen.simd_ge(height);
            views_blocked = higher_trees.select_mask(Mask::splat(true), views_blocked);
            if views_blocked.all() {
                break;
            }
        }
        let scenic_score = views.reduce_product();
        max_scenic_score = std::cmp::max(scenic_score, max_scenic_score);
    }

    println!("{}", max_scenic_score);
    Ok(())
}
