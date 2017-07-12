use std::env;
use std::io;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    if let Some(path) = env::args().nth(1) {
        let mut contents = String::new();
        match File::open(path) {
            Ok(mut file) => {
                match file.read_to_string(&mut contents) {
                    Ok(_) => {
                        match brainfuck::read(&contents) {
                            Ok(program) => {
                                let stdio = io::stdin();
                                let input = stdio.lock();
                                let output = io::stdout();
                                brainfuck::eval(program, input, output);
                            }
                            Err(msg) => println!("Read the file, but could not parse it: {}", msg),
                        }
                    }
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
    use std::io::{Read, Write}
;

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

    fn write_byte<W>(mut writer: W, byte: u8)
        where W: Write
    {
        write!(&mut writer, "{}", byte as char).expect("Unable to write");
    }

    fn read_byte<R>(reader: R) -> u8
        where R: Read
    {
        reader
            .bytes()
            .next()
            .and_then(|result| result.ok())
            .map(|byte| byte as u8)
            .unwrap_or(0)
    }

    pub fn eval<R, W>(program: Vec<Command>, mut reader: R, mut writer: W)
        where R: Read,
              W: Write
    {
        let mut memory: Vec<u8> = vec![0; 30000];
        let mut mem_pointer: usize = 0;
        let mut prog_pointer: usize = 0;
        let prog_len = program.len();

        while prog_pointer < prog_len {
            match program[prog_pointer] {
                Command::IncrementPointer => {
                    mem_pointer += 1;
                }
                Command::DecrementPointer => {
                    mem_pointer -= 1;
                }
                Command::IncrementValue => {
                    memory[mem_pointer] += 1;
                }
                Command::DecrementValue => {
                    memory[mem_pointer] -= 1;
                }
                Command::OutputValue => {
                    write_byte(&mut writer, memory[mem_pointer]);
                }
                Command::InputValue => {
                    memory[mem_pointer] = read_byte(&mut reader);
                }
                Command::ForwardsTo(next) => {
                    if memory[mem_pointer] == 0 {
                        prog_pointer = next;
                    }
                }
                Command::BackwardsTo(next) => {
                    if memory[mem_pointer] != 0 {
                        prog_pointer = next;
                    }
                }
            };

            prog_pointer += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use brainfuck::*;
    use brainfuck::Command::*;
    use std::io::Cursor;

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

    #[test]
    fn eval_empty() {
        let input = Cursor::new(&b""[..]);
        let mut output = Cursor::new(vec![]);

        match read("") {
            Ok(program) => eval(program, input, &mut output),
            Err(_) => assert!(false),
        }

        let expected = "";
        let actual = String::from_utf8(output.into_inner()).expect("Not UTF-8");

        assert_eq!(expected, actual);
    }

    #[test]
    fn eval_new_line() {
        let input = Cursor::new(&b""[..]);
        let mut output = Cursor::new(vec![]);

        match read("++++++++++.") {
            Ok(program) => eval(program, input, &mut output),
            Err(_) => assert!(false),
        }

        let expected = "\n";
        let actual = String::from_utf8(output.into_inner()).expect("Not UTF-8");

        assert_eq!(expected, actual);
    }

    #[test]
    fn eval_cat() {
        let input = Cursor::new(&b"Hello, World!"[..]);
        let mut output = Cursor::new(vec![]);

        match read(",[.,]") {
            Ok(program) => eval(program, input, &mut output),
            Err(_) => assert!(false),
        }

        let expected = "Hello, World!";
        let actual = String::from_utf8(output.into_inner()).expect("Not UTF-8");

        assert_eq!(expected, actual);
    }

    #[test]
    fn eval_hello_world() {
        let input = Cursor::new(&b""[..]);
        let mut output = Cursor::new(vec![]);

        match read("--------[>+>+++++>-->-->--->++++>------<<<<<<<-------]>.>---.>----..>-.>++++.>.>+++++++.<<<.+++.<.<-.>>>>+.") {
            Ok(program) => eval(program, input, &mut output),
            Err(_) => assert!(false),
        }

        let expected = "Hello, World!";
        let actual = String::from_utf8(output.into_inner()).expect("Not UTF-8");

        assert_eq!(expected, actual);
    }
}
