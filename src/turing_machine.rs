use super::data_structs::*;
use super::turing_tape::*;

pub struct Turing {
    pub rules: Vec<(TapeRuleKey, TapeRule)>,
    pub tape: Tape,

    pub pos: i32,
    pub state: TapeState,
}

impl Turing {
    // pub fn new(rules: Vec<(TapeRuleKey, TapeRule)>, tape: Tape, pos: i32) -> Self {
    //     Self { rules, tape, pos }
    // }

    pub fn random(n: u8) -> Self {
        Self {
            rules: Self::get_random_rules(n),
            tape: Tape::new(),
            pos: 0,
            state: TapeState::State(0),
        }
    }

    pub fn get_random_rules(n: u8) -> Vec<(TapeRuleKey, TapeRule)> {
        let mut rules = Vec::new();
        let mut states = vec![TapeState::Halt; n as usize];
        for i in 0..n - 1 {
            states[i as usize] = TapeState::State(i);
        }

        for symbol in TapeSymbol::iter() {
            for state in &states {
                let key = TapeRuleKey::new(symbol.clone(), state.clone());

                let instruction = TapeRule::new(
                    TapeSymbol::random(),
                    TapeShift::random(),
                    TapeState::random(n),
                );

                rules.push((key, instruction));
            }
        }

        rules
    }

    pub fn print_rules(&self) {
        println!();
        println!("RULES OF THE TURING MACHINE:");

        let symbols: Vec<TapeSymbol> = TapeSymbol::iter().cloned().collect();
        let num_symbols: usize = symbols.len();

        let num_states: usize = self.rules.len() / num_symbols;
        let mut states = vec![TapeState::Halt; num_states as usize];
        for i in 0..num_states - 1 {
            states[i as usize] = TapeState::State(i as u8);
        }

        for i in 0..=num_symbols {
            let mut line = String::new();
            for j in 0..=num_states {
                let subline = match (i, j) {
                    (0, 0) => "#".to_owned(),
                    (0, _) => format!("  {} ", states[j - 1]),
                    (_, 0) => format!("{}", symbols[i - 1]),
                    (_, _) => {
                        let key = TapeRuleKey::new(symbols[i as usize - 1], states[j as usize - 1]);
                        let value_index: Option<usize> = (0..self.rules.len())
                            .filter(|&i| self.rules[i].0 == key)
                            .next();
                        match value_index {
                            Some(i) => format!(" {}", self.rules[i].1),
                            None => "   ".to_owned(),
                        }
                    }
                };

                line += &subline;
            }
            println!("{}", line);
        }
        println!();
    }

    pub fn step(&mut self, num_steps: u8) -> Result<u8, u8> {
        let mut steps: u8 = 0;
        for _ in 0..num_steps {
            match self.step_once() {
                Ok(_) => steps += 1,
                Err(_) => return Err(steps),
            }
        }
        Ok(steps)
    }

    pub fn step_once(&mut self) -> Result<(), ()> {
        let symbol = self.tape.get(self.pos);
        let key = TapeRuleKey::new(symbol, self.state.clone());
        let rule = &self.rules.iter().find(|r| r.0 == key).unwrap().1;

        self.tape.set(self.pos, rule.symbol);
        self.pos += match rule.shift {
            TapeShift::Right => 1,
            TapeShift::Left => -1,
        };
        self.state = rule.state;

        if (self.pos + self.tape.zero_index as i32) < 0
            || (self.pos + self.tape.zero_index as i32) >= self.tape.tape.len() as i32
        {
            Tape::grow(&mut self.tape.tape, 1, rule.shift);
            if rule.shift == TapeShift::Left {
                self.tape.zero_index += 1;
            }
        }

        Ok(())
    }
}
