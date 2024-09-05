pub struct Solution {}

impl Solution {
    pub fn minimum_cost(s: String) -> i64 {
        let mut left = vec![[0; 2]; s.len()];
        left[0][0] = if s.as_bytes()[0] == b'0' { 0 } else { 1 };
        left[0][1] = if s.as_bytes()[0] == b'1' { 0 } else { 1 };

        for i in 1..s.len() {
            if s.as_bytes()[i] == b'0' {
                left[i][0] = left[i - 1][0];
                left[i][1] = left[i - 1][0] + i as i64 + 1;
            } else {
                left[i][0] = left[i - 1][1] + i as i64 + 1;
                left[i][1] = left[i - 1][1]
            }
        }

        let mut right = vec![[0; 2]; s.len()];
        right[s.len() - 1][0] = if s.as_bytes()[s.len() - 1] == b'0' {
            0
        } else {
            1
        };
        right[s.len() - 1][1] = if s.as_bytes()[s.len() - 1] == b'1' {
            0
        } else {
            1
        };

        for i in (0..s.len() - 1).rev() {
            if s.as_bytes()[i] == b'0' {
                right[i][0] = right[i + 1][0];
                right[i][1] = right[i + 1][0] + (s.len() - i) as i64;
            } else {
                right[i][0] = right[i + 1][1] + (s.len() - i) as i64;
                right[i][1] = right[i + 1][1]
            }
        }

        let mut result = right[0][0]
            .min(right[0][1])
            .min(left[s.len() - 1][0])
            .min(left[s.len() - 1][1]);
        for i in 1..s.len() - 1 {
            result = result
                .min(left[i][0] + right[i + 1][0])
                .min(left[i][1] + right[i + 1][1]);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2712() {
        assert_eq!(Solution::minimum_cost("0011".to_string()), 2);
        assert_eq!(Solution::minimum_cost("010101".to_string()), 9);
    }
}
