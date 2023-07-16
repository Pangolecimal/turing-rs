#![allow(dead_code, non_snake_case, unused_variables, unused_imports)]

pub mod data_structs;
use data_structs::*;

pub mod turing_machine;
use turing_machine::*;

pub mod turing_tape;
use turing_tape::*;

fn main() {
    /// ! HALT INCLUDED
    const NUM_STATES: u8 = 2 + 1;

    let mut turing = Turing::random(NUM_STATES);
    turing.print_rules();

    // turing.tape.set(0, TapeSymbol::One);
    // turing.tape.set(1, TapeSymbol::One);
    // turing.tape.set(-2, TapeSymbol::One);
    // turing.tape.set(3, TapeSymbol::One);

    let result = turing.step(9);
    println!("Result: {:?}", result);
    turing.tape.print_history();
}
