/*
 * Complete the 'gridChallenge' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING_ARRAY grid as parameter.
 */

use std::{char, usize};

#[allow(non_snake_case)]
fn gridChallenge(grid: &[String]) -> String {
    let l: usize = grid.len();
    let column_l: usize = grid[0].len();
    let mut new: Vec<String> = Vec::new();
    let mut temp_sort_compare: Vec<String> = Vec::new();
    let mut status: bool = true;

    for i in 0..l {
        let value = &grid[i];
        let mut sorted: Vec<char> = value.chars().collect();
        sorted.sort();
        new.push(sorted.into_iter().collect());
    }

    for i in 0..column_l {
        let mut temp_row = String::new();
        for j in 0..l {
            let row = &new[j];
            let temp: Vec<char> = row.chars().collect();
            temp_row.push(temp[i]);
        }

        let mut compare: Vec<char> = temp_row.chars().collect();
        compare.sort();
        temp_sort_compare.push(compare.into_iter().collect());

        if temp_row != temp_sort_compare[i] {
            status = false;
        }
    }

    if status {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

fn main() {
    let grid = ["abc".to_string(), "ade".to_string(), "efg".to_string()];
    let result = gridChallenge(&grid);
    println!("{}", result);
}
