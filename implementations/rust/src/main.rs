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
    pub type Pointer = u32;

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
        BackwardsTo(Pointer)
    }

    #[derive(PartialEq)]
    #[derive(Debug)]
    pub enum ReadResult {
        Program(Vec<Command>),
        UnmatchedJump
    }

    pub fn read(source: &str) -> ReadResult {
        let mut program: Vec<Command> = vec![];

        // find braces first

        return ReadResult::Program(program)
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
        let expected = Program(vec![
            IncrementValue,
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
            InputValue
        ]);
        let actual = read(source);
        assert_eq!(expected, actual);
    }

    #[test]
    fn read_garbage() {
        let source = ",.+lol,hey.>there<";
        let expected = Program(vec![
            InputValue,
            OutputValue,
            IncrementValue,
            InputValue,
            OutputValue,
            IncrementPointer,
            DecrementPointer
        ]);
        let actual = read(source);
        assert_eq!(expected, actual);
    }

    #[test]
    fn read_loop() {
        let source = "[->+<]";
        let expected = Program(vec![
            ForwardsTo(5),
            DecrementValue,
            IncrementPointer,
            IncrementValue,
            DecrementPointer,
            BackwardsTo(0)
        ]);
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
