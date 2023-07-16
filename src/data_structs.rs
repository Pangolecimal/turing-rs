use rand;
use std::slice::Iter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TapeSymbol {
    Zero,
    One,
}

impl TapeSymbol {
    pub fn random() -> Self {
        match rand::random::<bool>() {
            false => Self::Zero,
            true => Self::One,
        }
    }

    pub fn iter() -> Iter<'static, Self> {
        static SYMBOLS: [TapeSymbol; 2] = [TapeSymbol::Zero, TapeSymbol::One];
        SYMBOLS.iter()
    }
}

impl std::fmt::Display for TapeSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zero => write!(f, "0"),
            Self::One => write!(f, "1"),
            // Self::Zero => write!(f, "□"),
            // Self::One => write!(f, "■"),
            // Self::Zero => write!(f, "▒▒"),
            // Self::One => write!(f, "██"),
            // Self::Zero => write!(f, "\u{EE00}\u{EE02}"),
            // Self::One => write!(f, "\u{EE03}\u{EE05}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TapeShift {
    Right,
    Left,
}

impl TapeShift {
    pub fn random() -> Self {
        match rand::random::<bool>() {
            false => Self::Right,
            true => Self::Left,
        }
    }

    pub fn iter() -> Iter<'static, Self> {
        static SHIFTS: [TapeShift; 2] = [TapeShift::Right, TapeShift::Left];
        SHIFTS.iter()
    }
}

impl std::fmt::Display for TapeShift {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Right => write!(f, "R"),
            Self::Left => write!(f, "L"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TapeState {
    State(u8),
    Halt,
}

impl TapeState {
    pub fn random(n: u8) -> Self {
        let m = rand::random::<u8>() % n;
        match m {
            k if k < n - 1 => Self::State(k),
            k if k == n - 1 => Self::Halt,
            _ => unreachable!("`m` is somehow bigger than `n`"),
        }
    }
}

impl std::fmt::Display for TapeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::State(k) => match k {
                &n if n < 26 => {
                    write!(f, "{}", ('a' as u8 + n) as char)
                }
                _ => write!(f, "{}", k),
            },
            Self::Halt => write!(f, "H"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TapeRule {
    pub symbol: TapeSymbol,
    pub shift: TapeShift,
    pub state: TapeState,
}

impl TapeRule {
    pub fn new(symbol: TapeSymbol, shift: TapeShift, state: TapeState) -> Self {
        Self {
            symbol,
            shift,
            state,
        }
    }
}

impl std::fmt::Display for TapeRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.symbol, self.shift, self.state)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TapeRuleKey {
    pub symbol: TapeSymbol,
    pub state: TapeState,
}

impl TapeRuleKey {
    pub fn new(symbol: TapeSymbol, state: TapeState) -> Self {
        Self { symbol, state }
    }
}

impl std::fmt::Display for TapeRuleKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.symbol, self.state)
    }
}
