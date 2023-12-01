use std::io;

fn main() {
    let mut numbers: Vec<usize> = Vec::new();

    let mut lines = io::stdin().lines();
    while let Some(l) = lines.next() {
        let line = l.unwrap();

        let mut left_digit = 0;
        while !line.chars().nth(left_digit).unwrap().is_numeric() {
            left_digit += 1;
        }
        let left_num = line.chars().nth(left_digit).unwrap();

        let mut right_digit = line.len() - 1;
        while !line.chars().nth(right_digit).unwrap().is_numeric() {
            right_digit -= 1;
        }
        let right_num = line.chars().nth(right_digit).unwrap();

        let number = left_num.to_string() + &right_num.to_string();
        numbers.push(number.parse::<usize>().unwrap());
    }

    let result = numbers.iter().fold(0, |acc, elem| acc + elem);
    println!("{}", result);
}
