fn gray(n: u8) -> Vec<String> {
    if n == 0 {
        return vec![String::new()];
    }

    let prev = gray(n - 1);
    let mut result = Vec::new();

    for s in &prev {
        result.push(format!("0{}", s));
    }
    for s in &prev {
        result.push(format!("1{}", s));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_data = [
            (0, vec!["".to_string()]),
            (1, vec!["0", "1"].iter().map(|s| s.to_string()).collect()),
            (2, vec!["00", "01", "10", "11"].iter().map(|s| s.to_string()).collect()),
            (
                3,
                vec!["000", "001", "010", "011", "100", "101", "110", "111"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            ),
            (
                4,
                vec![
                    "0000", "0001", "0010", "0011", "0100", "0101", "0110", "0111",
                    "1000", "1001", "1010", "1011", "1100", "1101", "1110", "1111",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            ),
        ];

        test_data.iter().for_each(|(n, expected)| {
            assert_eq!(gray(*n), *expected);
        });
    }
}

fn main() {
    let result = gray(3);
    for code in result {
        println!("{}", code);
    }
}
