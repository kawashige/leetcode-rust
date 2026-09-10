pub struct Solution {}

impl Solution {
    pub fn vowel_consonant_score(s: String) -> i32 {
        let mut v = 0;
        let mut c = 0;

        for i in 0..s.len() {
            if s.as_bytes()[i].is_ascii_alphabetic() {
                if [b'a', b'e', b'i', b'o', b'u'].contains(&s.as_bytes()[i]) {
                    v += 1;
                } else {
                    c += 1;
                }
            }
        }

        if c == 0 { 0 } else { v / c }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3813() {
        assert_eq!(Solution::vowel_consonant_score("cooear".to_string()), 2);
        assert_eq!(Solution::vowel_consonant_score("axeyizou".to_string()), 1);
        assert_eq!(Solution::vowel_consonant_score("au 123".to_string()), 0);
    }
}

fn main() {}
