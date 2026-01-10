pub struct Solution {}

impl Solution {
    pub fn max_substring_length(s: String, k: i32) -> bool {
        let mut indices = vec![[s.len(), 0]; 26];
        for i in 0..s.len() {
            indices[(s.as_bytes()[i] - b'a') as usize][0] =
                indices[(s.as_bytes()[i] - b'a') as usize][0].min(i);
            indices[(s.as_bytes()[i] - b'a') as usize][1] =
                indices[(s.as_bytes()[i] - b'a') as usize][1].max(i);
        }

        let mut dp = vec![0; s.len() + 1];
        for i in 0..s.len() {
            let j = (s.as_bytes()[i] - b'a') as usize;

            for k in 0..26 {
                if (indices[k][0]..indices[k][1]).contains(&i) {
                    indices[k][0] = indices[k][0].min(indices[j][0]);
                    indices[k][1] = indices[k][1].max(indices[j][1]);
                }
            }

            if indices[j][1] == i {
                dp[i + 1] = (dp[indices[j][0]] + 1).max(dp[i]);
            } else {
                dp[i + 1] = dp[i];
            }
            if k <= dp[i + 1] && !(k == 1 && i == s.len() - 1) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3458() {
        assert!(!Solution::max_substring_length(
            "ddjlopbgngpoenkqktvuuvpygctyquoeqglszijjiifljfiswiladdfwzislzdd".to_string(),
            6
        ));
        assert!(Solution::max_substring_length("abcdbaefab".to_string(), 2));
        assert!(!Solution::max_substring_length("cdefdc".to_string(), 3));
        assert!(Solution::max_substring_length("abeabe".to_string(), 0));
    }
}
