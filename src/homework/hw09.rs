fn rotate(s: &str, n: &isize) -> String {
    let len = s.len() as isize;
    if len == 0 {
        return s.to_string();
    }

    let n = ((n % len) + len) % len; // нормалізація зсуву
    let n = n as usize;

    let chars: Vec<char> = s.chars().collect();
    let left = &chars[len as usize - n..];
    let right = &chars[..len as usize - n];
    [left, right].concat().iter().collect()
}

// Альтернативна сигнатура, яка відповідає тесту:
fn rotate2(s: &str, n: &isize) -> String {
    rotate(s, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "abcdefgh";
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];

        shifts.iter().for_each(|(n, exp)| {
            assert_eq!(rotate2(s, n), exp.to_string());
        });
    }
}
