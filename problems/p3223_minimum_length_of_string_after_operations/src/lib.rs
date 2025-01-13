pub struct Solution {}

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        s.as_bytes()
            .iter()
            .fold([0; 26], |mut count, b| {
                count[(b - b'a') as usize] += 1;
                count
            })
            .into_iter()
            .filter(|c| &0 < c)
            .map(|c| if c % 2 == 0 { 2 } else { 1 })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3223() {
        assert_eq!(Solution::minimum_length("abaacbcbb".to_string()), 5);
        assert_eq!(Solution::minimum_length("aa".to_string()), 2);
    }
}
