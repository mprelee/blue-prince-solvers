use itertools::Itertools;
use crate::ciphers::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum NumericCoreOps {
    Yellow,  // Subtract, LHS >= RHS
    Pink,    // Multiply
    Violet,  // Divide no remainder
}
use NumericCoreOps::*;

const NUMERIC_CORE_OP_COMBOS: &[(NumericCoreOps, NumericCoreOps, NumericCoreOps)] = &[
    (Yellow,   Pink,   Violet),
    (Yellow,   Violet, Pink),
    (Pink,     Yellow, Violet),
    (Pink,     Violet, Yellow),
    (Violet,   Yellow, Pink),
    (Violet,   Pink,   Yellow),
];

fn yellow_subtract(lhs: u64, rhs: u64) -> Option<u64> {
    if lhs >= rhs {
        Some(lhs - rhs)
    } else {
        None
    }
}

fn pink_multiply(lhs: u64, rhs: u64) -> Option<u64> {
    Some(lhs * rhs)
}

fn violet_divide(lhs: u64, rhs: u64) -> Option<u64> {
    if rhs != 0 && lhs % rhs == 0 {
        Some(lhs / rhs)
    } else {
        None
    }
}

impl NumericCoreOps {
    pub fn apply(&self, lhs: u64, rhs: u64) -> Option<u64> {
        match self {
            Yellow => yellow_subtract(lhs, rhs),
            Pink => pink_multiply(lhs, rhs),
            Violet => violet_divide(lhs, rhs),
        }
    }
}

pub fn numeric_core_word(word: &str) -> Option<char> {
    assert_eq!(word.len(), 4);
    let numeric: Vec<u64> = word.chars().map(|c| char_to_numeric(c).unwrap()).collect();
    let [a, b, c, d] = numeric[..] else { return None };

    let mut results: Vec<u64> = Vec::new();
    for (op1, op2, op3) in NUMERIC_CORE_OP_COMBOS {
        // Need to reverse order of ops when pink and violet 
        // are continguous and violet is first
        let result = match (op1, op2, op3) {
            (Violet, Pink, _)   => op2.apply(b, c).and_then(|x| op1.apply(a, x)).and_then(|x| op3.apply(x, d)),
            (_, Violet, Pink)   => op1.apply(a, b).and_then(|x| op3.apply(x, d)).and_then(|x| op2.apply(x, c)),
            (_, _, _) => op1.apply(a, b).and_then(|x| op2.apply(x, c)).and_then(|x| op3.apply(x, d)),
        };
        if result.is_some() {
            results.push(result.unwrap());
        }
    }
    if results.is_empty() {
        None
    } else {
        results.sort();
        numeric_to_char(results[0]).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_core_word() {
        assert_eq!(numeric_core_word("PIGS"), Some('S'));
    }
}
