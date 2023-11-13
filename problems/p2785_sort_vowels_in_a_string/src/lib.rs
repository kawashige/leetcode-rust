pub struct Solution {}

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let mut vowels = vec![];
        let mut vowels_index = vec![];
        for i in 0..s.len() {
            match s.as_bytes()[i] {
                b'A' | b'E' | b'I' | b'O' | b'U' | b'a' | b'e' | b'i' | b'o' | b'u' => {
                    vowels.push(s.as_bytes()[i]);
                    vowels_index.push(i);
                }
                _ => {}
            }
        }
        vowels.sort_unstable();

        let mut chars = s.chars().collect::<Vec<_>>();
        for i in 0..vowels_index.len() {
            chars[vowels_index[i]] = vowels[i] as char;
        }
        chars.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2785() {
        assert_eq!(
            Solution::sort_vowels("lEetcOde".to_string()),
            "lEOtcede".to_string()
        );
        assert_eq!(
            Solution::sort_vowels("lYmpH".to_string()),
            "lYmpH".to_string()
        );
    }
}
