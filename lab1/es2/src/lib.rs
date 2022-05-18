

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut tot = 0;
    let mut i = 1;
    for ch in code.chars().rev() {
        match ch {
            ' ' => continue,
            '0' => i += 1,
            '1'..='9' => {
                let num = ch.to_digit(10).unwrap();
                if i % 2 == 0 {
                    tot += num*2;
                    if num >= 5 {
                        tot -= 9;
                    }
                }
                else {
                    tot += num;
                }
                i += 1;
            },
            _ => return false,
        }
    }
    return (tot % 10) == 0 && i > 2;
}
