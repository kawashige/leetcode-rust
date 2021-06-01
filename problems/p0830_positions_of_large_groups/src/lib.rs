pub struct Solution {}

impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let s = s.as_bytes();
        let mut i = 0;
        let mut results = Vec::new();

        for j in 1..s.len() {
            if s[j] != s[j - 1] {
                if j - i > 2 {
                    results.push(vec![i as i32, j as i32 - 1]);
                }
                i = j;
            }
        }
        if s.len() - i > 2 {
            results.push(vec![i as i32, s.len() as i32 - 1]);
        }

        results
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0830() {
        assert_eq!(
            Solution::large_group_positions("abbxxxxzzyyyy".to_string()),
            vec![vec![3, 6], vec![9, 12]]
        );
        assert_eq!(
            Solution::large_group_positions("abbxxxxzzy".to_string()),
            vec![vec![3, 6]]
        );
        assert_eq!(
            Solution::large_group_positions("abc".to_string()),
            vec![] as Vec<Vec<i32>>
        );
        assert_eq!(
            Solution::large_group_positions("abcdddeeeeaabbbcd".to_string()),
            vec![vec![3, 5], vec![6, 9], vec![12, 14]]
        );
    }
}
