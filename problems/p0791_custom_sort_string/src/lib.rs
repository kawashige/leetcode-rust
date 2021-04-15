pub struct Solution {}

impl Solution {
    pub fn custom_sort_string(s: String, t: String) -> String {
        let order = s
            .chars()
            .enumerate()
            .fold([s.len(); 26], |mut order, (i, c)| {
                order[c as usize - 0x61] = i;
                order
            });

        let mut chars = t.chars().collect::<Vec<char>>();
        chars.sort_unstable_by_key(|c| order[*c as usize - 0x61]);
        chars.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0791() {
        assert_eq!(
            Solution::custom_sort_string("cba".to_string(), "abcd".to_string()),
            "cbad".to_string()
        );
    }
}
