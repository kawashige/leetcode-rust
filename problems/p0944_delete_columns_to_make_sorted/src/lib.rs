pub struct Solution {}

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let chars = strs
            .into_iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();

        (0..chars[0].len())
            .filter(|i| (1..chars.len()).any(|j| chars[j - 1][*i] > chars[j][*i]))
            .count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0944() {
        assert_eq!(
            Solution::min_deletion_size(vec![
                "cba".to_string(),
                "daf".to_string(),
                "ghi".to_string()
            ]),
            1
        );
        assert_eq!(
            Solution::min_deletion_size(vec!["a".to_string(), "b".to_string()]),
            0
        );
        assert_eq!(
            Solution::min_deletion_size(vec![
                "zyx".to_string(),
                "wvu".to_string(),
                "tsr".to_string()
            ]),
            3
        );
    }
}
