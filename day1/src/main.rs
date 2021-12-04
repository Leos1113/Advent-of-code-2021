use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("data/input.txt").expect("Unable to open this file"));

    let numbers: Vec<i32> = reader
        .lines()
        .flatten()
        .flat_map(|line| line.parse::<i32>())
        .collect();

    println!("part 1 -> times incremented: {}", part1(&numbers));
    println!("part 2 -> times incremented: {}", part2(&numbers));
}

fn part1(numbers: &[i32]) -> i32 {
    let mut times_incremented = 0;

    for (index, number) in numbers.iter().enumerate() {
        let curr_value: i32 = *number;
        let prev_value: i32 = if index == 0 {
            curr_value
        } else {
            numbers[index - 1]
        };

        if curr_value > prev_value {
            times_incremented += 1;
        }
    }

    times_incremented
}

fn part2(numbers: &[i32]) -> i32 {
    let mut times_incremented = 0;

    let mut prev_value: i32 = numbers.iter().take(3).sum();

    for index in 1..numbers.len() {
        if index + 3 > numbers.len() {
            break;
        }

        let curr_value: i32 = numbers[index..index + 3].iter().sum();

        if curr_value > prev_value {
            times_incremented += 1;
        }

        prev_value = curr_value;
    }

    times_incremented
}
