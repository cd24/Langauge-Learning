pub mod parsing;
pub mod types;

use std::collections::*;
use types::*;
use std::io::*;

#[allow(dead_code)]
pub fn run_program(program: String) -> Environment {
    match parsing::program_from(program).unwrap() {
        types::Instruction::Block(program) => {
            let mut env = new_environment();
            run(program, &mut env);
            return env;
        },
        _ => panic!("No program to run")
    }
}

pub fn new_environment() -> Environment {
    return Environment {
        pointer: 0,
        memory: HashMap::new()
    };
}

pub fn apply(instr: &Instruction, env: &mut Environment) {
    match instr {
        &Instruction::IncrementPointer   => increment_pointer(env),
        &Instruction::DecrementPointer   => decrement_pointer(env),
        &Instruction::IncrementValue     => increment_value(env),
        &Instruction::DecrementValue     => decrement_value(env),
        &Instruction::Print              => print(env),
        &Instruction::Read               => read(env),
        _ => println!("Warning: unexpected instruction to apply: {:?}", instr)
    }
}

pub fn run(program: BFBlock, environment: &mut Environment) {
    for instr in program.instructions {
        match instr {
            Instruction::Block(block) => {
                while environment.current_value() != 0 {
                    run(block.clone(), environment);
                }
            },
            to_execute @ _ => apply(&to_execute, environment)
        }
    }
}

fn increment_pointer(env: &mut Environment) {
    env.pointer += 1;
}

fn decrement_pointer(env: &mut Environment) {
    env.pointer -= 1;
}

fn increment_value(env: &mut Environment) {
    let next = env.current_value() + 1;
    env.memory.insert(env.pointer, next);
}

fn decrement_value(env: &mut Environment) {
    let next = env.current_value() - 1;
    env.memory.insert(env.pointer, next);
}

fn print(env: &mut Environment) {
    println!("{}", env.current_value() + 1);
}

fn read(env: &mut Environment) {
    let mut buffer: [u8; 1] = [0; 1];
    match stdin().read(&mut buffer) {
        Ok(count) => {
            if count == 1 {
                env.memory.insert(env.pointer, buffer[0] as i64);
            } else {
                println!("Warning: read incorrect amount of memory {}", count);
            }
        },
        Err(e) => println!("Error: encountered error while reading.\n{:?}", e)
    }
}

#[test]
fn memory_access() {
    let mut env = new_environment();
    assert!(env.current_value() == 0);
    env.memory.insert(0, 10);
    assert!(env.current_value() == 10)
}

#[test]
fn pointer_changes() {
    let mut env = new_environment();
    assert!(env.pointer == 0);
    increment_pointer(&mut env);
    assert!(env.pointer == 1);
    increment_pointer(&mut env);
    assert!(env.pointer == 2);
    decrement_pointer(&mut env);
    assert!(env.pointer == 1);
    decrement_pointer(&mut env);
    assert!(env.pointer == 0);
}