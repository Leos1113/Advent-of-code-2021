use std::str::FromStr;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Direction {
    Forward,
    Down,
    Up,
}

struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32
}

fn main() {
    let reader = BufReader::new(File::open("data/input.txt").expect("Unable to open this file"));

    let commands: Vec<String> = reader.lines().map(|l| l.expect("Could not parse line"))
    .collect();

    println!("Part 1 -> {}", part1(&commands));
    println!("Part 1 -> {}", part2(&commands));
}

fn get_intructions_from_command(command: &str) -> (Direction, i32) {
    let splitted_command: Vec<&str> = command.split_whitespace().collect();

    let direction: Direction = Direction::from_str(splitted_command[0]).unwrap();
    let steps: i32 = splitted_command[1].parse().unwrap();

    (direction, steps)
}

fn part1(commands: &[String]) -> i32 {
    let mut position: Position = Position::new(0, 0, 0);

    for command in commands.iter() {
        let (direction, steps) = get_intructions_from_command(command);

        match direction {
            Direction::Forward => position.horizontal += steps,
            Direction::Up => position.depth -= steps,
            Direction::Down => position.depth += steps,
        }

    }
    
    position.horizontal * position.depth
}

fn part2(commands: &[String]) -> i32 {
    let mut position: Position = Position::new(0, 0, 0);

    for command in commands.iter() {

        let (direction, steps) = get_intructions_from_command(command);

        match direction {
            Direction::Forward => {
                position.horizontal += steps;
                position.depth += position.aim * steps;
            },
            Direction::Up => position.aim -= steps,
            Direction::Down => position.aim += steps,
        }

    }
    
    position.horizontal * position.depth
}

impl Position {
    fn new(horizontal: i32, depth: i32, aim: i32) -> Position {
        Position { horizontal, depth, aim}
    }
}

impl FromStr for Direction {
    type Err = (); 

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "forward" => Ok(Direction::Forward),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => Err(())
        }
    }
}


#[cfg(test)]
mod test_day2 {
    use std::string::String;
    #[test]
    fn test_part1() {
        let sample_input: Vec<String> = vec!["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"].iter().map(|s| s.to_string()).collect();
        let sample_output = 150;
        assert_eq!(crate::part1(&sample_input), sample_output);
    }

    #[test]
    fn test_part2() {
        let sample_input: Vec<String> = vec!["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"].iter().map(|s| s.to_string()).collect();
        let sample_output = 900;
        assert_eq!(crate::part2(&sample_input), sample_output);
    }
}