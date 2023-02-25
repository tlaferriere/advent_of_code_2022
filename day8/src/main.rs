#![feature(portable_simd)]

use array2d::Array2D;
use itertools::izip;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;
use std::simd::{i32x4, SimdPartialEq, SimdPartialOrd};

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

    let mut visible_trees = HashSet::new();
    let rows = forest.as_rows();
    let columns = forest.as_columns();
    for (i_row, (row, col)) in zip(rows, columns).enumerate() {
        let j_col = i_row;
        let mut max_height = i32x4::splat(-1);
        for (
            (j_row, height_row),
            (j_row_rev, height_row_rev),
            (i_col, height_col),
            (i_col_rev, height_col_rev),
        ) in izip!(
            row.iter().enumerate(),
            row.iter().enumerate().rev(),
            col.iter().enumerate(),
            col.iter().enumerate().rev()
        ) {
            let height = i32x4::from([
                *height_row as i32,
                *height_row_rev as i32,
                *height_col as i32,
                *height_col_rev as i32,
            ]);
            let are_higher = height.simd_gt(max_height);
            if are_higher.test(0) {
                visible_trees.insert((i_row, j_row));
            }
            if are_higher.test(1) {
                visible_trees.insert((i_row, j_row_rev));
            }
            if are_higher.test(2) {
                visible_trees.insert((i_col, j_col));
            }
            if are_higher.test(3) {
                visible_trees.insert((i_col_rev, j_col));
            }
            max_height = are_higher.select(height, max_height);
            if max_height.simd_eq(i32x4::splat(9)).all() {
                break;
            }
        }
    }

    println!("{}", visible_trees.len());
    Ok(())
}
