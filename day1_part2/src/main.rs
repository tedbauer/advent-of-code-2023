use anyhow::{anyhow, Result};
use std::io;

static DIGIT_STRINGS: [&str; 20] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
    "five", "six", "seven", "eight", "nine",
];

fn first_match(s: &str) -> Result<usize, anyhow::Error> {
    let mut idx: usize = 0;

    while idx < s.len() {
        for digit_string in DIGIT_STRINGS {
            if matches_string(&s[idx..], digit_string) {
                return digit_from_string(digit_string);
            }
        }
        idx += 1;
    }
    Err(anyhow!("found no first match"))
}

fn last_match(s: &str) -> Result<usize, anyhow::Error> {
    let mut idx: usize = s.len();

    while idx >= 0 {
        for digit_string in DIGIT_STRINGS {
            if matches_string(&s[idx..], digit_string) {
                return digit_from_string(digit_string);
            }
        }
        idx -= 1;
    }
    Err(anyhow!("found no last match for {}", s))
}

fn digit_from_string(s: &str) -> Result<usize, anyhow::Error> {
    match s {
        "one" => Ok(1),
        "two" => Ok(2),
        "three" => Ok(3),
        "four" => Ok(4),
        "five" => Ok(5),
        "six" => Ok(6),
        "seven" => Ok(7),
        "eight" => Ok(8),
        "nine" => Ok(9),
        n => n.parse().map_err(|_| anyhow!("failed to parse {}", s)),
    }
}

fn matches_string(s: &str, target: &str) -> bool {
    let mut idx: usize = 0;

    loop {
        let s_char = s.chars().nth(idx);
        let target_char = target.chars().nth(idx);

        match (s_char, target_char) {
            (None, None) => break,
            (Some(_), None) => break,
            (None, Some(_)) => return false,
            (Some(s_c), Some(t_c)) => {
                if !s_c.eq(&t_c) {
                    return false;
                }
            }
        }
        idx += 1;
    }
    true
}

fn main() {
    let mut numbers: Vec<usize> = Vec::new();

    let mut lines = io::stdin().lines();
    while let Some(l) = lines.next() {
        let line = l.unwrap();

        let (fst, snd) = (first_match(&line).unwrap(), last_match(&line).unwrap());
        let number_string = fst.to_string() + &snd.to_string();
        numbers.push(number_string.parse::<usize>().unwrap());
    }

    let result = numbers.iter().fold(0, |acc, elem| acc + elem);
    println!("{}", result);
}
