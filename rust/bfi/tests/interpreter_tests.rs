extern crate bfi;

//use bfi::parsing;
//use bfi::types;

#[test]
fn simple_val_increment_program() {
    let mut env = bfi::run_program("+++".to_string());
    assert!(env.current_value() == 3);
}

#[test]
fn simple_pointer_increment_program() {
    let mut env = bfi::run_program("+>+>+>".to_string());
    assert!(env.current_value() == 0);
    assert!(env.value_at(0) == 1);
    assert!(env.value_at(1) == 1);
    assert!(env.value_at(2) == 1);
    assert!(env.pointer == 3);
}

#[test]
fn simple_nested_program() {
    let mut skip_block  = bfi::run_program("<+<+<[++++<]".to_string());
    let mut run_block   = bfi::run_program("<+<+<+[++++<]".to_string());

    assert!(skip_block.value_at(-3) == 0);
    assert!(skip_block.value_at(-2) == 1);
    assert!(skip_block.value_at(-1) == 1);

    assert!(run_block.value_at(-3) == 5);
    assert!(run_block.value_at(-2) == 1);
    assert!(run_block.value_at(-1) == 1);
}

#[test]
fn simple_loop_test() {
    let mut loop_test = bfi::run_program("++>++>++[<]-".to_string());

    assert!(loop_test.value_at(0) == 2, "0 should be 2 not {}", loop_test.value_at(0));
    assert!(loop_test.value_at(1) == 2, "0 should be 2 not {}", loop_test.value_at(1));
    assert!(loop_test.value_at(2) == 2, "0 should be 2 not {}", loop_test.value_at(2));

    assert!(loop_test.value_at(-1) == -1, "pointer should be -1 not {}", loop_test.pointer);
    assert!(loop_test.pointer == -1i64);
}

