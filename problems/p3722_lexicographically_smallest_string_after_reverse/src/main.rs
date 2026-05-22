pub struct Solution {}

impl Solution {
    pub fn lex_smallest(s: String) -> String {
        let mut result = s.clone();

        for i in 0..s.len() {
            let new_s = s[..=i]
                .chars()
                .rev()
                .chain(s[i + 1..].chars())
                .collect::<String>();
            if new_s < result {
                result = new_s;
            }

            let new_s = s[..=i]
                .chars()
                .chain(s[i + 1..].chars().rev())
                .collect::<String>();
            if new_s < result {
                result = new_s;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3722() {
        assert_eq!(
            Solution::lex_smallest("dcab".to_string()),
            "acdb".to_string()
        );
        assert_eq!(
            Solution::lex_smallest("abba".to_string()),
            "aabb".to_string()
        );
        assert_eq!(Solution::lex_smallest("zxy".to_string()), "xzy".to_string());
    }
}

fn main() {}
