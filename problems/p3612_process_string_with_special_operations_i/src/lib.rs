pub struct Solution {}

impl Solution {
    pub fn process_str(s: String) -> String {
        let mut result = String::new();

        for c in s.chars() {
            match c {
                '*' => {
                    if !result.is_empty() {
                        result.pop();
                    }
                }
                '#' => {
                    result += &result.clone();
                }
                '%' => result = result.chars().rev().collect(),
                _ => {
                    result.push(c);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3612() {
        assert_eq!(Solution::process_str("a#b%*".to_string()), "ba".to_string());
        assert_eq!(Solution::process_str("z*#".to_string()), "".to_string());
    }
}
