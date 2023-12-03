use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::io::{BufRead, BufReader};

fn part_one(input: Vec<String>) {
    let mut result = 0;

    let symbol_positions = gen_symbol_positions(input.clone());
    for (row, row_string) in input.iter().enumerate() {
        let mut idx = 0;
        loop {
            match row_string.chars().nth(idx) {
                Some(c) if c.is_numeric() => {
                    let start = idx;
                    while let Some(n) = row_string.chars().nth(idx) {
                        if !n.is_numeric() {
                            break;
                        }
                        idx += 1;
                    }
                    let end = idx - 1;

                    if adjacent_to_symbol(row as i64, start as i64, end as i64, &symbol_positions) {
                        let num = row_string[start..end + 1].parse::<usize>().unwrap();
                        result += num;
                    }
                }
                Some(_) => (),
                None => break,
            }
            idx += 1;
        }
    }
    println!("{}", result);
}

fn adjacent_to_symbol(row: i64, start: i64, end: i64, symbols: &HashSet<(i64, i64)>) -> bool {
    for col in start..(end + 1) {
        if symbols.contains(&(row + 1, col)) {
            return true;
        }
        if symbols.contains(&(row + 1, col + 1)) {
            return true;
        }
        if symbols.contains(&(row + 1, col - 1)) {
            return true;
        }

        if symbols.contains(&(row, col)) {
            return true;
        }
        if symbols.contains(&(row, col + 1)) {
            return true;
        }
        if symbols.contains(&(row, col - 1)) {
            return true;
        }

        if symbols.contains(&(row - 1, col)) {
            return true;
        }
        if symbols.contains(&(row - 1, col + 1)) {
            return true;
        }
        if symbols.contains(&(row - 1, col - 1)) {
            return true;
        }
    }
    false
}

fn gen_symbol_positions(input: Vec<String>) -> HashSet<(i64, i64)> {
    let mut result = HashSet::new();
    for (row, row_string) in input.iter().enumerate() {
        for (col, col_char) in row_string.chars().enumerate() {
            if !col_char.is_numeric() && !col_char.eq(&'.') {
                result.insert((row as i64, col as i64));
            }
        }
    }
    result
}

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let lines = input
        .lines()
        .map(|result| result.unwrap())
        .collect::<Vec<String>>();

    part_one(lines);
}
