pub struct Solution {}

impl Solution {
    pub fn longest_dup_substring(s: String) -> String {
        let mut suffix = (0..s.len()).map(|i| &s[i..]).collect::<Vec<_>>();
        suffix.sort_unstable();

        let mut max_l = 0;
        let mut max_i = 0;

        for i in 1..suffix.len() {
            let l = suffix[i - 1]
                .as_bytes()
                .iter()
                .zip(suffix[i].as_bytes().iter())
                .take_while(|(b1, b2)| b1 == b2)
                .count();

            if max_l < l {
                max_l = l;
                max_i = i;
            }
        }

        suffix[max_i].chars().take(max_l).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1044() {
        assert_eq!(
            Solution::longest_dup_substring("banana".to_string()),
            "ana".to_string()
        );
        assert_eq!(
            Solution::longest_dup_substring("abcd".to_string()),
            "".to_string()
        );
    }
}
