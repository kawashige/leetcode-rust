pub struct Solution {}

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut left = vec![0; s.len()];
        let mut right = vec![0; s.len()];
        left[0] = 1 << (s.as_bytes()[0] - b'a');
        right[s.len() - 1] = 1 << (s.as_bytes()[s.len() - 1] - b'a');

        for i in 1..s.len() {
            left[i] = left[i - 1] | 1 << (s.as_bytes()[i] - b'a');
            right[s.len() - 1 - i] =
                right[right.len() - i] | 1 << (s.as_bytes()[s.len() - 1 - i] - b'a');
        }

        let mut counts = [0_i32; 26];

        for i in 1..s.len() - 1 {
            counts[(s.as_bytes()[i] - b'a') as usize] |= left[i - 1] & right[i + 1];
        }

        counts
            .into_iter()
            .map(|count| count.count_ones() as i32)
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1930() {
        assert_eq!(
            Solution::count_palindromic_subsequence("aabca".to_string()),
            3
        );
        assert_eq!(
            Solution::count_palindromic_subsequence("adc".to_string()),
            0
        );
        assert_eq!(
            Solution::count_palindromic_subsequence("bbcbaba".to_string()),
            4
        );
    }
}
