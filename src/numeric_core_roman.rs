fn split_into_four_parts_str(s: &str) -> Vec<[&str; 4]> {
    assert!(s.len() >= 4);
    let n = s.len();
    (1..n)
        .combinations(3)
        .filter_map(|indices| {
            let (i, j, k) = (indices[0], indices[1], indices[2]);
            if i < j && j < k && k < n {
                Some([
                    &s[0..i],
                    &s[i..j],
                    &s[j..k],
                    &s[k..n],
                ])
            } else {
                None
            }
        })
        .collect()
}


pub fn roman_to_int(s: &str) -> u64 {
    let mut result = 0;
    let mut prev = 0;
    for c in s.chars() {
        let value = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };
        if value > prev {
            result -= prev;
        } else {
            result += prev;
        }
        prev = value;
    }
    result
}