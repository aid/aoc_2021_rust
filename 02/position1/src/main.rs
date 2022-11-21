use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Command {
    Forward(i32),
    Down(i32),
    Up(i32)
}

impl Command {
    fn new(command: &str) -> Command {
        let split = command.split(' ').collect::<Vec<&str>>();
        if split.len() != 2 {
            panic!("Could not parse command string: {}", command);
        }
        let value = i32::from_str(split[1]).expect("Could not value parse string to i32");
        match split[0] {
            "forward"   => Command::Forward(value),
            "down"      => Command::Down(value),
            "up"        => Command::Up(value),
            _           => panic!("Unknown command {}", split[0])
        }
    }
}

fn load_commands(path: &str) -> Vec<Command> {
    let f = File::open(path).expect("Could not open file");
    BufReader::new(f)
        .lines()
        .map(|i| i.unwrap())
        .map(|i| Command::new(&i))
        .collect::<Vec<Command>>()
}

fn calculate_position(commands: Vec<Command>) -> (i32, i32) {
    let mut position: (i32, i32) = (0,0);
    for command in commands.iter() {
        match command {
            Command::Forward(value) => position = (position.0 + value, position.1),
            Command::Down(value)    => position = (position.0, position.1 + value),
            Command::Up(value)      => position = (position.0, position.1 - value),
        }
    }
    position
}

fn main() {
    let commands = load_commands("input.txt");
    let position = calculate_position(commands);
    println!("Final position = {:?}", position);
    println!("Product = {}", position.0 * position.1);

}
// Final position = (2085, 785)
// Product = 1636725

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_commands() {
        let commands = load_commands("sample_input.txt");
        let position = calculate_position(commands);
        assert_eq!(position, (15,10));
    }
}