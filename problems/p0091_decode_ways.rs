pub struct Solution {}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        use std::collections::HashMap;
        fn is_valid(chars: Vec<char>) -> bool {
            if chars[0] == '0' {
                false
            } else {
                let n = chars.iter().collect::<String>().parse::<i32>().unwrap();
                0 < n && n < 27
            }
        }
        fn count(chars: Vec<char>, memo: &mut HashMap<String, i32>) -> i32 {
            let s = chars.iter().collect::<String>();
            match memo.get(&s) {
                Some(n) => *n,
                None => match chars.len() {
                    0 => 1,
                    1 => match is_valid(chars) {
                        true => 1,
                        false => 0,
                    },
                    _ => {
                        let mut result = 0;
                        if is_valid(chars[0..1].to_vec()) {
                            result += count(chars[1..].to_vec(), memo);
                        }
                        if is_valid(chars[0..2].to_vec()) {
                            result += count(chars[2..].to_vec(), memo);
                        }
                        memo.insert(s, result);
                        result
                    }
                },
            }
        }
        match s.len() {
            0 => 0,
            _ => count(s.chars().collect::<Vec<char>>(), &mut HashMap::new()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0091() {
        assert_eq!(2, Solution::num_decodings("12".to_string()));
        assert_eq!(3, Solution::num_decodings("226".to_string()));
        assert_eq!(2, Solution::num_decodings("236".to_string()));
        assert_eq!(0, Solution::num_decodings("0".to_string()));
        assert_eq!(3981312, Solution::num_decodings("9371597631128776948387197132267188677349946742344217846154932859125134924241649584251978418763151253".to_string()));
    }
}
