use anyhow::{anyhow, Result};
use std::io;
use std::collections::HashSet;

fn main() {
    let mut result = 0;

    let mut lines = io::stdin().lines();
    while let Some(l) = lines.next() {
        let line = l.unwrap();

        let winning_numbers: HashSet<usize> = line
            .split(":")
            .nth(1)
            .unwrap()
            .trim()
            .split("|")
            .nth(0)
            .unwrap()
            .trim()
            .split(" ")
            .filter(|num| !num.trim().is_empty())
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        let numbers: HashSet<usize> = line
            .split(":")
            .nth(1)
            .unwrap()
            .trim()
            .split("|")
            .nth(1)
            .unwrap()
            .trim()
            .split(" ")
            .filter(|num| !num.trim().is_empty())
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        let matches = (winning_numbers.intersection(&numbers).count());
        if matches > 0 {
            result += 1 << (matches-1);
        }
    }
    println!("{}", result);
}
