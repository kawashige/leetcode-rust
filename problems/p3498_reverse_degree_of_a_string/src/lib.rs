pub struct Solution {}

impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        s.as_bytes()
            .iter()
            .zip(1..)
            .map(|(b, i)| i * (27 - (b - b'a' + 1)) as i32)
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3498() {
        assert_eq!(Solution::reverse_degree("abc".to_string()), 148);
        assert_eq!(Solution::reverse_degree("zaza".to_string()), 160);
    }
}
