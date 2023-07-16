use super::data_structs::*;

//? THEORETICAL TAPE: [..., -3, -2, -1, 0, 1, 2, 3, ...] ==>
//? - [ ] 1) [0, 1, -1, 2, -2, 3, -3, ...]
//? - [x] 2) [-n, ..., -3, -2, -1, 0, 1, 2, 3, ..., p]
//? Example: [-2, -1, 0, 1] => zero_index = 2 => get(-1) <=> zero_index + (-1)

#[derive(Debug, Clone)]
pub struct Tape {
    pub tape_history: Vec<Vec<TapeSymbol>>,
    pub tape: Vec<TapeSymbol>,
    pub zero_history: Vec<u32>,
    pub zero_index: u32,
}

impl Tape {
    pub fn new() -> Self {
        Self {
            tape_history: vec![vec![TapeSymbol::Zero]],
            tape: vec![TapeSymbol::Zero],
            zero_history: vec![0],
            zero_index: 0,
        }
    }

    pub fn grow(tape: &mut Vec<TapeSymbol>, n: i32, shift: TapeShift) {
        match shift {
            TapeShift::Left => {
                (0..n).for_each(|_| tape.insert(0, TapeSymbol::Zero));
            }
            TapeShift::Right => {
                (0..n + 1).for_each(|_| tape.push(TapeSymbol::Zero));
            }
        }
    }

    pub fn set(&mut self, position: i32, symbol: TapeSymbol) {
        let index: i32 = self.zero_index as i32 + position;

        let change: i32 = match index {
            n if n < 0 => {
                let diff = -n;
                Tape::grow(&mut self.tape, diff, TapeShift::Left);
                self.zero_index += diff as u32;
                self.tape[0] = symbol;

                n
            }
            p if p as usize >= self.tape.len() => {
                let diff = p - self.tape.len() as i32;
                Tape::grow(&mut self.tape, diff, TapeShift::Right);
                let last_index = self.tape.len() - 1;
                self.tape[last_index] = symbol;

                diff
            }
            n => {
                self.tape[n as usize] = symbol;

                0
            }
        };

        if change < 0 {
            for tape in &mut self.tape_history {
                Tape::grow(tape, -change, TapeShift::Left);
            }
        }

        self.tape_history.push(self.tape.clone());
        self.zero_history.push(self.zero_index);
    }

    pub fn get(&self, position: i32) -> TapeSymbol {
        let index: i32 = self.zero_index as i32 + position;

        match index {
            n if n < 0 || n >= self.tape.len() as i32 => TapeSymbol::Zero,
            _ => self.tape[index as usize],
        }
    }

    pub fn print_history(&self) {
        println!();
        println!("TAPE HISTORY: (zero_index from the first `1`)");

        let longest_tape_len = self
            .tape_history
            .last()
            .unwrap_or(&self.tape_history[0])
            .len();

        self.tape_history
            .iter()
            .enumerate()
            .for_each(|(index, tape)| {
                print!("tape_{index}: ");
                let mut tape_symbols: String = "".to_owned();

                for symbol in tape.iter() {
                    tape_symbols += &symbol.to_string();
                }

                print!("{:<lp$}", tape_symbols, lp = longest_tape_len);
                print!("  zero_index: {}\n", self.zero_history[index]);
            });
    }
}
