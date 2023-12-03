use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader};

fn part_two(input: Vec<String>) {
    let stars_to_adjacent_numbers = gen_stars_to_adjacent_numbers(input);
    let result = stars_to_adjacent_numbers
        .iter()
        .filter(|(_, adjacent_numbers)| adjacent_numbers.len() == 2)
        .fold(0, |acc, (_, adjacent_numbers)| {
            let n1 = adjacent_numbers.get(0).unwrap();
            let n2 = adjacent_numbers.get(1).unwrap();
            acc + (n1 * n2)
        });
    println!("{}", result);
}

fn gen_stars_to_adjacent_numbers(input: Vec<String>) -> HashMap<(i64, i64), Vec<usize>> {
    let star_positions = gen_star_positions(input.clone());
    let mut result: HashMap<(i64, i64), Vec<usize>> = HashMap::new();
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
                    let num = row_string[start..end + 1].parse::<usize>().unwrap();

                    let adjacent_stars = gen_adjacent_stars(
                        row as i64,
                        start as i64,
                        end as i64,
                        star_positions.clone(),
                    );
                    for (star_row, star_col) in adjacent_stars {
                        if let Some(numbers) = result.get_mut(&(star_row, star_col)) {
                            numbers.push(num);
                        } else {
                            result.insert((star_row, star_col), vec![num]);
                        }
                    }
                }
                Some(_) => (),
                None => break,
            }
            idx += 1;
        }
    }
    result
}

fn gen_adjacent_stars(
    row: i64,
    start: i64,
    end: i64,
    star_positions: HashSet<(i64, i64)>,
) -> HashSet<(i64, i64)> {
    let mut result = HashSet::new();
    for col in start..(end + 1) {
        if star_positions.contains(&(row + 1, col)) {
            result.insert((row + 1, col));
        }
        if star_positions.contains(&(row + 1, col + 1)) {
            result.insert((row + 1, col + 1));
        }
        if star_positions.contains(&(row + 1, col - 1)) {
            result.insert((row + 1, col - 1));
        }

        if star_positions.contains(&(row, col)) {
            result.insert((row, col));
        }
        if star_positions.contains(&(row, col + 1)) {
            result.insert((row, col + 1));
        }
        if star_positions.contains(&(row, col - 1)) {
            result.insert((row, col - 1));
        }

        if star_positions.contains(&(row - 1, col)) {
            result.insert((row - 1, col));
        }
        if star_positions.contains(&(row - 1, col + 1)) {
            result.insert((row - 1, col + 1));
        }
        if star_positions.contains(&(row - 1, col - 1)) {
            result.insert((row - 1, col - 1));
        }
    }
    result
}

fn gen_star_positions(input: Vec<String>) -> HashSet<(i64, i64)> {
    let mut result = HashSet::new();
    for (row, row_string) in input.iter().enumerate() {
        for (col, col_char) in row_string.chars().enumerate() {
            if col_char.eq(&'*') {
                result.insert((row as i64, col as i64));
            }
        }
    }
    result
}

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

    //part_one(lines.clone());
    part_two(lines);
}
