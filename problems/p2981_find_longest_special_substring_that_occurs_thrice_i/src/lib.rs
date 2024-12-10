pub struct Solution {}

impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let mut count = [[0; 51]; 26];
        for i in 0..s.len() {
            count[(s.as_bytes()[i] - b'a') as usize][1] += 1;
            for j in (0..i).rev() {
                if s.as_bytes()[j] != s.as_bytes()[i] {
                    break;
                }
                count[(s.as_bytes()[i] - b'a') as usize][i - j + 1] += 1;
            }
        }
        let mut result = -1;
        for i in 0..count.len() {
            for j in (1..count[0].len()).rev() {
                if 2 < count[i][j] {
                    result = result.max(j as i32)
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2981() {
        assert_eq!(Solution::maximum_length("aaaa".to_string()), 2);
        assert_eq!(Solution::maximum_length("abcdef".to_string()), -1);
        assert_eq!(Solution::maximum_length("abcaba".to_string()), 1);
    }
}
