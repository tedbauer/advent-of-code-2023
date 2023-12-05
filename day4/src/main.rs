use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};
use std::io;

fn part_two() {
    let lines: Vec<String> = io::stdin().lines().map(|l| l.unwrap()).collect();
    let mut card_number = 1;

    let mut card_counts: HashMap<usize, usize> = HashMap::new();

    for n in 1..(lines.len()+1) {
        card_counts.insert(n, 1);
    }

    for line in lines {
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

        for num in (card_number + 1)..(card_number + matches+1) {
            let increment = card_counts.get(&card_number).unwrap_or(&1);
            match card_counts.get(&num) {
                Some(c) => {
                    card_counts.insert(num, c + increment)
                },
                None => {
                    panic!("should never happen")
                },
            };
        }
        card_number += 1;
    }

    let result = card_counts.iter().fold(0, |acc, (_, count)| acc + count);
    println!("{}", result);
}

fn part_one() {
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
            result += 1 << (matches - 1);
        }
    }
    println!("{}", result);
}

fn main() {
    //part_one();
    part_two();
}
