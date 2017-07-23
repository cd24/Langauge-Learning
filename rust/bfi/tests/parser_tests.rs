extern crate bfi;

use bfi::types::Instruction as i;
use bfi::types::ParseError as err;
use bfi::types::BFBlock;
use bfi::parsing;
use parsing::instruction_from as convert;

use std::str::Chars;

fn instr_equals( i1: Result<i, err>, i2: i) {
    match i1 {
        Err(e) => panic!("Should not return error {:?}", e),
        Ok(val) => assert!(val == i2, "{:?} does not equal {:?}", val, i2)
    }
}

fn parse_throws(input: Result<i, err>, expected: err) {
    match input {
        Ok(_) => panic!("Should throw"),
        Err(e) => assert!(expected == e)
    }
}

fn should_not_parse(input: char) {
    parse_throws(convert(input), err::UnknownCharacter(input))
}

fn reads_instruction_single_scope(chars: &mut Chars, expected: i) {
    match parsing::handle_character(chars, true) {
        Ok(value) => assert!(value == expected, "{:?} does not equal {:?}", value, expected),
        Err(error) => panic!("Unexpected error: {:?}", error)
    }
}

fn programs_match(prog: String, answer: Vec<i>) {
    let prog_dup = prog.clone();
    let answers = BFBlock { instructions: answer };
    match parsing::program_from(prog) {
        Ok(result) => {
            match result {
                i::Block(block) => {
                    assert!(block == answers);
                },
                ins @ _ => panic!("Wrong type of instruction {:?}", ins)
            }
        },
        Err(error) => panic!("Should parse program {}, error: {:?}", prog_dup, error)
    }
}

#[test]
fn converts_operations() {
    instr_equals(convert('>'), i::IncrementPointer);
    instr_equals(convert('<'), i::DecrementPointer);
    instr_equals(convert('+'), i::IncrementValue);
    instr_equals(convert('-'), i::DecrementValue);
    instr_equals(convert('.'), i::Print);
    instr_equals(convert(','), i::Read);
}

#[test]
fn does_not_convert_alphabetic() {
    should_not_parse('a');
    should_not_parse('b');
    should_not_parse('c');
    should_not_parse('d');
    should_not_parse('e');
    should_not_parse('f');
    should_not_parse('g');
    should_not_parse('h');
    should_not_parse('i');
    should_not_parse('j');
    should_not_parse('k');
    should_not_parse('l');
    should_not_parse('m');
    should_not_parse('n');
    should_not_parse('o');
    should_not_parse('p');
    should_not_parse('q');
    should_not_parse('r');
    should_not_parse('s');
    should_not_parse('t');
    should_not_parse('u');
    should_not_parse('v');
    should_not_parse('w');
    should_not_parse('x');
    should_not_parse('y');
    should_not_parse('z');
}

#[test]
fn does_not_convert_numeric() {
    should_not_parse('1');
    should_not_parse('2');
    should_not_parse('3');
    should_not_parse('4');
    should_not_parse('5');
    should_not_parse('6');
    should_not_parse('7');
    should_not_parse('8');
    should_not_parse('9');
    should_not_parse('0');
}

#[test]
fn does_not_convert_symbolic() {
    should_not_parse('!');
    should_not_parse('@');
    should_not_parse('#');
    should_not_parse('$');
    should_not_parse('%');
    should_not_parse('^');
    should_not_parse('&');
    should_not_parse('*');
    should_not_parse('(');
    should_not_parse(')');
    should_not_parse('`');
    should_not_parse('~');
    should_not_parse('{');
    should_not_parse('}');
    should_not_parse('\\');
    should_not_parse('\'');
    should_not_parse(';');
    should_not_parse(':');
    should_not_parse('/');
    should_not_parse('?');
}

#[test]
fn reads_root_scope() {
    let mut chars = "+>+>+".chars();
    reads_instruction_single_scope(&mut chars, i::IncrementValue);
    reads_instruction_single_scope(&mut chars, i::IncrementPointer);
    reads_instruction_single_scope(&mut chars, i::IncrementValue);
    reads_instruction_single_scope(&mut chars, i::IncrementPointer);
    reads_instruction_single_scope(&mut chars, i::IncrementValue);
    reads_instruction_single_scope(&mut chars, i::Close);
}

#[test]
fn reads_single_scope_program() {
    let prog = "+>-<+".to_string();
    let answer = vec!(i::IncrementValue,
                      i::IncrementPointer,
                      i::DecrementValue,
                      i::DecrementPointer,
                      i::IncrementValue);
    programs_match(prog, answer);
}

#[test]
fn reads_single_nested_scope_program() {
    let prog = "+[-]".to_string();
    let answer = vec!(
        i::IncrementValue,
        i::Block(
            BFBlock {
                instructions: vec!(i::DecrementValue)
            }
        )
    );
    programs_match(prog, answer);
}

#[test]
fn reads_double_nested_scope_program() {
    let prog = "+[-[++]]".to_string();
    let answer = vec!(
        i::IncrementValue,
        i::Block(
            BFBlock { instructions: vec!(
                i::DecrementValue,
                i::Block(
                    BFBlock { instructions: vec!(
                        i::IncrementValue,
                        i::IncrementValue
                    ) }
                )
            ) }
        )
    );
    programs_match(prog, answer);
}

#[test]
fn reads_only_scope_program() {
    let prog = "[--]".to_string();
    let answer = vec!(
        i::Block( BFBlock {
            instructions: vec!(i::DecrementValue, i::DecrementValue)
        })
    );
    programs_match(prog, answer);
}

#[test]
fn reads_multiple_concurrent_blocks() {
    let prog = "[+][-]".to_string();
    let answer = vec!(
        i::Block( BFBlock {instructions: vec!(i::IncrementValue)}),
        i::Block( BFBlock {instructions: vec!(i::DecrementValue)})
    );
    programs_match(prog, answer);
}

#[test]
fn reads_empty_program() {
    let prog = "".to_string();
    let answer = vec!();
    programs_match(prog, answer);
}

#[test]
fn reads_empty_block() {
    let prog = "[]".to_string();
    let answer = vec!(
        i::Block( BFBlock {instructions: vec!() })
    );
    programs_match(prog, answer);
}

#[test]
fn handles_unexpected_end() {
    let prog1 = "[".to_string();
    let prog2 = "++[--][".to_string();
    let prog3 = "++[+++[+++]".to_string();

    match parsing::program_from(prog1) {
        Ok(val) => panic!("Should not derive value from imbalanced program: {:?}", val),
        Err(e) => assert!(e == err::UnexpectedEndOfFile)
    }
    match parsing::program_from(prog2) {
        Ok(val) => panic!("Should not derive value from imbalanced program: {:?}", val),
        Err(e) => assert!(e == err::UnexpectedEndOfFile)
    }
    match parsing::program_from(prog3) {
        Ok(val) => panic!("Should not derive value from imbalanced program: {:?}", val),
        Err(e) => assert!(e == err::UnexpectedEndOfFile)
    }
}