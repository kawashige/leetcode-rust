pub struct Solution {}

impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let count = s.as_bytes().iter().fold([0; 26], |mut count, b| {
            count[(b - b'a') as usize] += 1;
            count
        });
        *count
            .iter()
            .filter(|c| &&0 < c && **c % 2 == 1)
            .max()
            .unwrap() as i32
            - *count
                .iter()
                .filter(|c| &&0 < c && **c % 2 == 0)
                .min()
                .unwrap() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3442() {
        assert_eq!(Solution::max_difference("aaaaabbc".to_string()), 3);
        assert_eq!(Solution::max_difference("abcabcab".to_string()), 1);
    }
}
