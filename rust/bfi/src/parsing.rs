use std::string::String;
use std::str::Chars;
use types::*;

pub fn instruction_from(character: char) -> Result<Instruction, ParseError> {
    match character {
        '>' => Ok(Instruction::IncrementPointer),
        '<' => Ok(Instruction::DecrementPointer),
        '+' => Ok(Instruction::IncrementValue),
        '-' => Ok(Instruction::DecrementValue),
        '.' => Ok(Instruction::Print),
        ',' => Ok(Instruction::Read),
        _   => Err(ParseError::UnknownCharacter(character))
    }
}

pub fn program_from(input: String) -> Result<Instruction, ParseError> {
    let mut characters = input.chars();
    return program(&mut characters, true);
}

pub fn program(input: &mut Chars, can_end: bool) -> Result<Instruction, ParseError> {
    let mut vec = Vec::new();
    loop {
        match handle_character(input, can_end) {
            Ok(instr) => {
                match instr {
                    Instruction::Close => {
                        return Ok( Instruction::Block(BFBlock { instructions: vec }));
                    },
                    other @ _ => vec.push(other)
                }
            },
            e @ Err(_) => {
                return e;
            }
        }
    }
}

pub fn handle_character(input: &mut Chars, can_end: bool) -> Result<Instruction, ParseError> {
    match input.next() {
        Some('[') => program(input, false),
        Some(']') => Ok(Instruction::Close),
        Some(char) => instruction_from(char),
        None => {
            if !can_end {
                return Err(ParseError::UnexpectedEndOfFile);
            } else {
                Ok(Instruction::Close)
            }
        }
    }
}