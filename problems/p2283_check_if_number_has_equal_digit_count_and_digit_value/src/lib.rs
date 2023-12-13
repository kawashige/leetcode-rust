pub struct Solution {}

impl Solution {
    pub fn digit_count(num: String) -> bool {
        let count = num.as_bytes().iter().fold([0; 10], |mut count, b| {
            count[(b - b'0') as usize] += 1;
            count
        });
        (0..num.len()).all(|i| count[i] == num.as_bytes()[i] - b'0')
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2283() {
        assert!(Solution::digit_count("1210".to_string()));
        assert!(!Solution::digit_count("030".to_string()));
    }
}
