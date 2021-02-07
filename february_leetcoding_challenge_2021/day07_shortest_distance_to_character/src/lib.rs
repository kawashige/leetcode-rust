pub struct Solution {}

impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let bytes = s.as_bytes();
        let mut results = Vec::with_capacity(bytes.len());
        let mut distance = 10_001;
        for i in 0..bytes.len() {
            distance = if bytes[i] == c as u8 { 0 } else { distance + 1 };
            results.push(distance);
        }
        distance = 10_001;
        for i in (0..bytes.len()).rev() {
            distance = if bytes[i] == c as u8 { 0 } else { distance + 1 };
            results[i] = std::cmp::min(distance, results[i]);
        }
        results
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day07() {
        assert_eq!(
            Solution::shortest_to_char("loveleetcode".to_string(), 'e'),
            vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]
        );
        assert_eq!(
            Solution::shortest_to_char("aaab".to_string(), 'b'),
            vec![3, 2, 1, 0]
        );
    }
}
