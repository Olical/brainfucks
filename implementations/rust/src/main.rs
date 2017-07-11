use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    if let Some(path) = env::args().nth(1) {
        let mut contents = String::new();
        match File::open(path) {
            Ok(mut file) => {
                match file.read_to_string(&mut contents) {
                    Ok(_) => println!("Would read and eval here."),
                    Err(_) => println!("Found the file but failed to read it."),
                }
            }
            Err(_) => println!("Failed to open the file for reading."),
        };
    } else {
        println!("Please provide a path to some brainfuck source as the first argument.")
    }
}

mod brainfuck {
    #[derive(PartialEq)]
    #[derive(Debug)]
    pub enum Command {
        IncrementPointer,
        DecrementPointer,
        IncrementValue,
        DecrementValue,
        OutputValue,
        InputValue,
        ForwardsTo(usize),
        BackwardsTo(usize),
    }

    pub fn read(source: &str) -> Result<Vec<Command>, &'static str> {
        let mut forward_pointers: Vec<usize> = vec![];
        let mut backward_pointers: Vec<usize> = vec![];
        let mut position: usize = 0;
        let mut program: Vec<Command> = source
            .chars()
            .filter_map(|token| {
                let command = match token {
                    '>' => Some(Command::IncrementPointer),
                    '<' => Some(Command::DecrementPointer),
                    '+' => Some(Command::IncrementValue),
                    '-' => Some(Command::DecrementValue),
                    '.' => Some(Command::OutputValue),
                    ',' => Some(Command::InputValue),
                    '[' => {
                        forward_pointers.push(position);
                        Some(Command::BackwardsTo(position))
                    }
                    ']' => {
                        backward_pointers.push(position);
                        Some(Command::ForwardsTo(position))
                    }
                    _ => None,
                };

                if command.is_some() {
                    position += 1
                }

                command
            })
            .collect();

        if forward_pointers.len() == backward_pointers.len() {
            for (a, b) in forward_pointers.iter().zip(backward_pointers) {
                program.swap(*a, b)
            }

            Ok(program)
        } else {
            Err("Found an unmatched square brace.")
        }
    }

    // pub fn eval(program: Vec<Command>) {
    // }
}

#[cfg(test)]
mod tests {
    use brainfuck::*;
    use brainfuck::Command::*;

    #[test]
    fn read_empty() {
        let source = "";
        let expected: Vec<Command> = vec![];

        match read(source) {
            Ok(actual) => assert_eq!(expected, actual),
            Err(_) => assert!(false),
        };
    }

    #[test]
    fn read_simple() {
        let source = "+>-<.,<.-+>,";
        let expected = vec![IncrementValue,
                            IncrementPointer,
                            DecrementValue,
                            DecrementPointer,
                            OutputValue,
                            InputValue,
                            DecrementPointer,
                            OutputValue,
                            DecrementValue,
                            IncrementValue,
                            IncrementPointer,
                            InputValue];

        match read(source) {
            Ok(actual) => assert_eq!(expected, actual),
            Err(_) => assert!(false),
        };
    }

    #[test]
    fn read_garbage() {
        let source = ",.+lol,hey.>there<";
        let expected = vec![InputValue,
                            OutputValue,
                            IncrementValue,
                            InputValue,
                            OutputValue,
                            IncrementPointer,
                            DecrementPointer];

        match read(source) {
            Ok(actual) => assert_eq!(expected, actual),
            Err(_) => assert!(false),
        };
    }

    #[test]
    fn read_loop() {
        let source = "[->+<]";
        let expected = vec![ForwardsTo(5),
                            DecrementValue,
                            IncrementPointer,
                            IncrementValue,
                            DecrementPointer,
                            BackwardsTo(0)];

        match read(source) {
            Ok(actual) => assert_eq!(expected, actual),
            Err(_) => assert!(false),
        };
    }

    #[test]
    fn read_bad_loop() {
        let source = "[,.[,.>+-][./.[]]";
        let expected = "Found an unmatched square brace.";

        match read(source) {
            Ok(_) => assert!(false),
            Err(actual) => assert_eq!(expected, actual),
        };
    }

    // #[test]
    // fn eval_empty() {
    //     let program = vec![];
    //     eval(program);
    //     assert_eq!(expected, output)
    // }
}
