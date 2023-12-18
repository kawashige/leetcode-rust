pub struct Solution {}

impl Solution {
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        let s_count = s.as_bytes().iter().fold([0; 26], |mut count, b| {
            count[(b - b'a') as usize] += 1;
            count
        });
        let target_count = target.as_bytes().iter().fold([0; 26], |mut count, b| {
            count[(b - b'a') as usize] += 1;
            count
        });
        target
            .as_bytes()
            .iter()
            .map(|b| s_count[(b - b'a') as usize] / target_count[(b - b'a') as usize])
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2287() {
        assert_eq!(
            Solution::rearrange_characters("ilovecodingonleetcode".to_string(), "code".to_string()),
            2
        );
        assert_eq!(
            Solution::rearrange_characters("abcba".to_string(), "abc".to_string()),
            1
        );
        assert_eq!(
            Solution::rearrange_characters("abbaccaddaeea".to_string(), "aaaaa".to_string()),
            1
        );
    }
}
