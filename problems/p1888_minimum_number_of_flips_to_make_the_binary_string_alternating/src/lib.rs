pub struct Solution {}

impl Solution {
    pub fn min_flips(s: String) -> i32 {
        if s.len() % 2 == 0 {
            let count = (0..s.len())
                .filter(|i| (s.as_bytes()[*i] - b'0') as usize % 2 == i % 2)
                .count();
            count.min(s.len() - count) as i32
        } else {
            let mut left = vec![0; s.len()];
            for i in 0..s.len() {
                if 0 < i {
                    left[i] += left[i - 1];
                }
                if (s.as_bytes()[i] - b'0') as usize % 2 != i % 2 {
                    left[i] += 1;
                }
            }
            let mut right = vec![0; s.len()];
            for i in (0..s.len()).rev() {
                if i < s.len() - 1 {
                    right[i] += right[i + 1];
                }
                if (s.as_bytes()[i] - b'0') as usize % 2 != i % 2 {
                    right[i] += 1;
                }
            }

            let mut min_swap = std::usize::MAX;
            for i in 0..s.len() {
                let swap = (if i == 0 { 0 } else { i - left[i - 1] } + right[i])
                    .min(if i == 0 { 0 } else { left[i - 1] } + s.len() - i - right[i]);
                min_swap = min_swap.min(swap);
            }

            min_swap as i32
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1888() {
        assert_eq!(Solution::min_flips("111000".to_string()), 2);
        assert_eq!(Solution::min_flips("010".to_string()), 0);
        assert_eq!(Solution::min_flips("1110".to_string()), 1);
    }
}
