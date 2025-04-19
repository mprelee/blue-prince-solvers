pub fn char_to_numeric(c: char) -> Result<u64, String> {
    match c.to_ascii_uppercase() {
        'A' => Ok(1),
        'B' => Ok(2),
        'C' => Ok(3),
        'D' => Ok(4),
        'E' => Ok(5),
        'F' => Ok(6),
        'G' => Ok(7),
        'H' => Ok(8),
        'I' => Ok(9),
        'J' => Ok(10),
        'K' => Ok(11),
        'L' => Ok(12),
        'M' => Ok(13),
        'N' => Ok(14),
        'O' => Ok(15),
        'P' => Ok(16),
        'Q' => Ok(17),
        'R' => Ok(18),
        'S' => Ok(19),
        'T' => Ok(20),
        'U' => Ok(21),
        'V' => Ok(22),
        'W' => Ok(23),
        'X' => Ok(24),
        'Y' => Ok(25),
        'Z' => Ok(26),
        _ => Err(format!("Invalid character: {}", c)),
    }
}
pub fn numeric_to_char(n: u64) -> Result<char, String> {
    match n {
        1 => Ok('A'),
        2 => Ok('B'),
        3 => Ok('C'),
        4 => Ok('D'),
        5 => Ok('E'),
        6 => Ok('F'),
        7 => Ok('G'),
        8 => Ok('H'),
        9 => Ok('I'),
        10 => Ok('J'),
        11 => Ok('K'),
        12 => Ok('L'),
        13 => Ok('M'),
        14 => Ok('N'),
        15 => Ok('O'),
        16 => Ok('P'),
        17 => Ok('Q'),
        18 => Ok('R'),
        19 => Ok('S'),
        20 => Ok('T'),
        21 => Ok('U'),
        22 => Ok('V'),
        23 => Ok('W'),
        24 => Ok('X'),
        25 => Ok('Y'),
        26 => Ok('Z'),
        _ => Err(format!("Invalid numeric: {}", n)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_to_numeric_uppercase() {
        for c in 'A'..='Z' {
            assert_eq!(char_to_numeric(c), Ok(c as u64 - 'A' as u64 + 1));
        }
    }

    #[test]
    fn test_char_to_numeric_lowercase() {
        for c in 'a'..='z' {
            assert_eq!(char_to_numeric(c), Ok(c as u64 - 'a' as u64 + 1));
        }
    }
}

