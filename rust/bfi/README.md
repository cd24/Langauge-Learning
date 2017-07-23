# Rust BrainFuck Interpreter
[BrainFuck](https://esolangs.org/wiki/Brainfuck#Hello.2C_World.21) is an esoteric programming 
language which works in a tape-based memory system. I wrote this strictly to practice 
my Rust. 

Significant limitations:
- No comments. The parser doesn't strip out comments before parsing the provided input. When 
passing input to the `run_program` function ensure that comments and newlines have been removed


#### Usage

This library can be used with a single call, `bfi::run_program(String)` when the provided input
is cleaned of whitespace and comments.

Example:
```rust
bfi::run_program("++>++>++[<]-".to_string());
```
This will return an `Environment` object where the memory at `0`, `1`, and `2` are all `2` and the
value at `-1` is `-1`

#### Future improvements
- Pretty printing for programs and state (debug should print a runable version of the program)
- Support for comments
- Migrate the `run_program` and `run` commands to use references instead of owning the
program block.