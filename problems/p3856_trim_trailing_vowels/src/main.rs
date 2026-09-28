pub struct Solution {}

impl Solution {
    pub fn trim_trailing_vowels(s: String) -> String {
        s.trim_end_matches(|c| ['a', 'e', 'i', 'o', 'u'].contains(&c))
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3856() {
        assert_eq!(
            Solution::trim_trailing_vowels("idea".to_string()),
            "id".to_string()
        );
        assert_eq!(
            Solution::trim_trailing_vowels("day".to_string()),
            "day".to_string()
        );
        assert_eq!(
            Solution::trim_trailing_vowels("aeiou".to_string()),
            "".to_string()
        );
    }
}

fn main() {}
