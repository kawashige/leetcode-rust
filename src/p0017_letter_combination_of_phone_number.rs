pub struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        use std::collections::HashMap;
        let mut chars = HashMap::new();
        chars.insert('2', vec!["a", "b", "c"]);
        chars.insert('3', vec!["d", "e", "f"]);
        chars.insert('4', vec!["g", "h", "i"]);
        chars.insert('5', vec!["j", "k", "l"]);
        chars.insert('6', vec!["m", "n", "o"]);
        chars.insert('7', vec!["p", "q", "r", "s"]);
        chars.insert('8', vec!["t", "u", "v"]);
        chars.insert('9', vec!["w", "x", "y", "z"]);

        digits
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .map(|x| chars.get(&x).unwrap())
            .fold(Vec::new(), |acc, x| {
                if acc.len() == 0 {
                    return x.iter().map(|x| x.to_string()).collect();
                }
                let mut tmp = Vec::new();
                for i in 0..acc.len() {
                    for j in 0..x.len() {
                        tmp.push(format!("{}{}", acc[i], x[j]))
                    }
                }
                tmp
            })
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0017() {
        assert_eq!(
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"],
            Solution::letter_combinations("23".to_string())
        );
    }
}
