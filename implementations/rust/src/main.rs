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
    pub type Pointer = usize;

    #[derive(PartialEq)]
    #[derive(Debug)]
    pub enum Command {
        IncrementPointer,
        DecrementPointer,
        IncrementValue,
        DecrementValue,
        OutputValue,
        InputValue,
        ForwardsTo(Pointer),
        BackwardsTo(Pointer),
    }

    #[derive(PartialEq)]
    #[derive(Debug)]
    pub enum ReadResult {
        Program(Vec<Command>),
        UnmatchedJump,
    }

    pub fn read(source: &str) -> ReadResult {
        let mut loops: Vec<usize> = vec![];
        let mut loop_closes: usize = 0;
        let mut position: usize = 0;
        let program: Vec<Command> = source
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
                        loops.push(position);
                        Some(Command::BackwardsTo(position))
                    }
                    ']' => {
                        loop_closes += 1;
                        Some(Command::ForwardsTo(position))
                    }
                    _ => None,
                };

                if command.is_some() {
                    position += 1;
                }

                command
            })
            .collect();

        if loops.len() != loop_closes {
            ReadResult::UnmatchedJump
        } else {
            ReadResult::Program(program)
        }
    }

    // pub fn eval(program: Vec<Command>) {
    // }
}

#[cfg(test)]
mod tests {
    use brainfuck::*;
    use brainfuck::Command::*;
    use brainfuck::ReadResult::*;

    #[test]
    fn read_empty() {
        let source = "";
        let expected = Program(vec![]);
        let actual = read(source);
        assert_eq!(expected, actual);
    }

    #[test]
    fn read_simple() {
        let source = "+>-<.,<.-+>,";
        let expected = Program(vec![IncrementValue,
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
                                    InputValue]);
        let actual = read(source);
        assert_eq!(expected, actual);
    }

    #[test]
    fn read_garbage() {
        let source = ",.+lol,hey.>there<";
        let expected = Program(vec![InputValue,
                                    OutputValue,
                                    IncrementValue,
                                    InputValue,
                                    OutputValue,
                                    IncrementPointer,
                                    DecrementPointer]);
        let actual = read(source);
        assert_eq!(expected, actual);
    }

    #[test]
    fn read_loop() {
        let source = "[->+<]";
        let expected = Program(vec![ForwardsTo(5),
                                    DecrementValue,
                                    IncrementPointer,
                                    IncrementValue,
                                    DecrementPointer,
                                    BackwardsTo(0)]);
        let actual = read(source);
        assert_eq!(expected, actual);
    }

    #[test]
    fn read_bad_loop() {
        let source = "[,.[,.>+-][./.[]]";
        let expected = UnmatchedJump;
        let actual = read(source);
        assert_eq!(expected, actual);
    }
}
