pub struct Solution {}

impl Solution {
    pub fn longest_beautiful_substring(word: String) -> i32 {
        let mut run_length = Vec::new();
        let mut len = 0;
        for i in 0..word.len() {
            len += 1;
            if i == word.len() - 1 || word.as_bytes()[i] != word.as_bytes()[i + 1] {
                run_length.push((word.as_bytes()[i], len));
                len = 0;
            }
        }

        let target = [b'a', b'e', b'i', b'o', b'u'];
        run_length
            .windows(target.len())
            .map(|r| {
                if (0..target.len()).all(|i| target[i] == r[i].0) {
                    r.iter().map(|(_, c)| c).sum()
                } else {
                    0
                }
            })
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1839() {
        assert_eq!(
            Solution::longest_beautiful_substring("aeiaaioaaaaeiiiiouuuooaauuaeiu".to_string()),
            13
        );
        assert_eq!(
            Solution::longest_beautiful_substring("aeeeiiiioooauuuaeiou".to_string()),
            5
        );
        assert_eq!(Solution::longest_beautiful_substring("a".to_string()), 0);
    }
}
