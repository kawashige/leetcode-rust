pub struct Solution {}

impl Solution {
    pub fn min_deletion(s: String, k: i32) -> i32 {
        let mut count = s
            .as_bytes()
            .iter()
            .fold(vec![0; 26], |mut count, b| {
                count[(b - b'a') as usize] += 1;
                count
            })
            .into_iter()
            .filter(|c| c > &0)
            .collect::<Vec<_>>();
        count.sort_unstable();
        count[..count.len() - (k as usize).min(count.len())]
            .iter()
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3545() {
        assert_eq!(Solution::min_deletion("abc".to_string(), 2), 1);
        assert_eq!(Solution::min_deletion("aabb".to_string(), 2), 0);
        assert_eq!(Solution::min_deletion("yyyzz".to_string(), 1), 2);
    }
}
