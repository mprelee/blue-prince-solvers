use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum NumericCoreOps {
    Yellow,  // Subtract, LHS >= RHS
    Pink,    // Multiply
    Violet,  // Divide no remainder
}

const NUMERIC_CORE_OP_COMBOS: &[(NumericCoreOps, NumericCoreOps, NumericCoreOps)] = &[
    (NumericCoreOps::Yellow,   NumericCoreOps::Pink, NumericCoreOps::Violet),
    (NumericCoreOps::Yellow,   NumericCoreOps::Violet, NumericCoreOps::Pink),
    (NumericCoreOps::Pink,     NumericCoreOps::Yellow, NumericCoreOps::Violet),
    (NumericCoreOps::Pink,     NumericCoreOps::Violet, NumericCoreOps::Yellow),
    (NumericCoreOps::Violet,   NumericCoreOps::Yellow, NumericCoreOps::Pink),
    (NumericCoreOps::Violet,   NumericCoreOps::Pink, NumericCoreOps::Yellow),
];

fn yellow_subtract(lhs: u32, rhs: u32) -> Option<u32> {
    if lhs >= rhs {
        Some(lhs - rhs)
    } else {
        None
    }
}

fn pink_multiply(lhs: u32, rhs: u32) -> u32 {
    lhs * rhs
}

fn violet_divide(lhs: u32, rhs: u32) -> Option<u32> {
    if rhs != 0 && lhs % rhs == 0 {
        Some(lhs / rhs)
    } else {
        None
    }
}

impl NumericCoreOps {
    pub fn apply(&self, lhs: u32, rhs: u32) -> Option<u32> {
        match self {
            NumericCoreOps::Yellow => yellow_subtract(lhs, rhs),
            NumericCoreOps::Pink => Some(pink_multiply(lhs, rhs)),
            NumericCoreOps::Violet => violet_divide(lhs, rhs),
        }
    }
}


fn split_into_four_parts(x: u32) -> Vec<[u32; 4]> {
    let s = x.to_string();
    assert!(s.len() >= 4);
    let n = s.len();
    (1..n)
        .combinations(3)
        .filter_map(|indices| {
            let (i, j, k) = (indices[0], indices[1], indices[2]);
            if i < j && j < k && k < n {
                Some([
                    s[0..i].parse().unwrap(),
                    s[i..j].parse().unwrap(),
                    s[j..k].parse().unwrap(),
                    s[k..n].parse().unwrap(),
                ])
            } else {
                None
            }
        })
        .collect()
}

pub fn numeric_core(x: u32) -> Option<u32> {
    if x == 0 {
        return None;
    } else if x < 1000 {
        Some(x)
    } else {
        let candidates = split_into_four_parts(x);
        let mut results: Vec<u32> = Vec::new();
        for candidate in candidates {
            for (op1, op2, op3) in NUMERIC_CORE_OP_COMBOS {
                if let Some(result) = op1.apply(candidate[0], candidate[1])
                    .and_then(|x| op2.apply(x, candidate[2]))
                    .and_then(|x| op3.apply(x, candidate[3])) {
                        if result > 0 {
                            results.push(result);
                        }
                }
            }
        }
        
        if results.is_empty() {
            None
        } else {
            // sort smallest to largest
            results.sort();
            // Recurse
            numeric_core(results[0])
        } }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_core_small() {
        assert_eq!(numeric_core(123), Some(123));
    }

    #[test]
    fn test_numeric_core_866455() {
        assert_eq!(numeric_core(86455), Some(18));
    }

    #[test]
    fn test_numeric_core_3614() {
        assert_eq!(numeric_core(3614), Some(14));
    }
}
