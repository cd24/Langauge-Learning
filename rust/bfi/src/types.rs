use std::collections;

#[derive(Clone, PartialEq, Debug)]
pub enum Instruction {
    IncrementPointer,
    DecrementPointer,
    IncrementValue,
    DecrementValue,
    Print,
    Read,
    Close,
    Block(BFBlock)
}

#[derive(Clone, PartialEq, Debug)]
pub enum ParseError {
    UnexpectedEndOfFile,
    UnknownCharacter(char),
    Generic(String)
}

#[derive(Clone, PartialEq, Debug)]
pub struct BFBlock {
    pub instructions: Vec<Instruction>
}

pub struct Environment {
    pub pointer: i64,
    pub memory: collections::HashMap<i64, i64>
}

impl Environment {
    #[allow(dead_code)]
    pub fn current_value(&mut self) -> i64 {
        *self.memory.entry(self.pointer).or_insert(0)
    }

    pub fn value_at(&mut self, position: i64) -> i64 {
        *self.memory.entry(position).or_insert(0)
    }
}