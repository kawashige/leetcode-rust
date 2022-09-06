pub struct Solution {}

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let bytes = s.as_bytes();

        let mut right_a = vec![0; s.len() + 1];
        let mut left_b = vec![0; s.len() + 1];

        for i in 0..bytes.len() {
            left_b[i + 1] += left_b[i];
            if bytes[i] == b'b' {
                left_b[i + 1] += 1;
            }
        }
        for i in (1..bytes.len()).rev() {
            right_a[i - 1] += right_a[i];
            if bytes[i] == b'a' {
                right_a[i - 1] += 1;
            }
        }

        right_a
            .into_iter()
            .zip(left_b.into_iter())
            .map(|(a, b)| a + b)
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1653() {
        assert_eq!(Solution::minimum_deletions("aababbab".to_string()), 2);
        assert_eq!(Solution::minimum_deletions("bbaaaaabb".to_string()), 2);
    }
}
