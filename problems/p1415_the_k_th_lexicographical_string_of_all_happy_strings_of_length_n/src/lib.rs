pub struct Solution {}

impl Solution {
    pub fn generate(happy_strings: Vec<String>, n: usize) -> Vec<String> {
        let mut target = happy_strings;
        while target[0].len() < n {
            target = target
                .iter()
                .map(|s| {
                    ('a'..='c').flat_map(move |c| {
                        if s.ends_with(c) {
                            None
                        } else {
                            Some(s.chars().chain(std::iter::once(c)).collect())
                        }
                    })
                })
                .flatten()
                .collect();
        }
        target
    }

    pub fn get_happy_string(n: i32, k: i32) -> String {
        let mut happy_strings = Self::generate(vec!["".to_string()], n as usize);
        happy_strings.sort_unstable();
        if happy_strings.len() < k as usize {
            "".to_string()
        } else {
            happy_strings[k as usize - 1].clone()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1415() {
        assert_eq!(Solution::get_happy_string(1, 3), "c".to_string());
        assert_eq!(Solution::get_happy_string(1, 4), "".to_string());
        assert_eq!(Solution::get_happy_string(3, 9), "cab".to_string());
    }
}
