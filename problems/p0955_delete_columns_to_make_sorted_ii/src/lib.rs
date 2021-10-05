pub struct Solution {}

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let chars = strs
            .into_iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut r = 0;

        let mut ok = vec![false; chars.len()];
        let mut ok_count = 1;

        for j in 0..chars[0].len() {
            if (1..chars.len()).any(|i| !ok[i] && chars[i - 1][j] > chars[i][j]) {
                r += 1;
            } else {
                for i in 1..chars.len() {
                    if !ok[i] && chars[i - 1][j] < chars[i][j] {
                        ok[i] = true;
                        ok_count += 1;
                    }
                }
                if ok_count == chars.len() {
                    break;
                }
            }
        }

        r
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0955() {
        assert_eq!(
            Solution::min_deletion_size(vec!["ca".to_string(), "bb".to_string(), "ac".to_string()]),
            1
        );
        assert_eq!(
            Solution::min_deletion_size(vec!["xc".to_string(), "yb".to_string(), "za".to_string()]),
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
        assert_eq!(
            Solution::min_deletion_size(vec![
                "xga".to_string(),
                "xfb".to_string(),
                "yfa".to_string()
            ]),
            1
        );
        assert_eq!(
            Solution::min_deletion_size(vec!["jwkwdc".to_string(), "etukoz".to_string()]),
            2
        );
    }
}
