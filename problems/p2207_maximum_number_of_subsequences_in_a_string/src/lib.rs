pub struct Solution {}

impl Solution {
    pub fn maximum_subsequence_count(text: String, pattern: String) -> i64 {
        let mut right_count = vec![0; text.len()];
        let mut counts = [0; 2];
        let mut result = 0;
        for i in (0..text.len()).rev() {
            if i + 1 < text.len() {
                right_count[i] += right_count[i + 1];
                if text.as_bytes()[i + 1] == pattern.as_bytes()[1] {
                    right_count[i] += 1;
                }
            }
            if text.as_bytes()[i] == pattern.as_bytes()[0] {
                counts[0] += 1;
                result += right_count[i];
            } else if text.as_bytes()[i] == pattern.as_bytes()[1] {
                counts[1] += 1;
            }
        }

        result + counts[0].max(counts[1]) as i64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2207() {
        assert_eq!(
            Solution::maximum_subsequence_count("abdcdbc".to_string(), "ac".to_string()),
            4
        );
        assert_eq!(
            Solution::maximum_subsequence_count("aabb".to_string(), "ab".to_string()),
            6
        );
    }
}
