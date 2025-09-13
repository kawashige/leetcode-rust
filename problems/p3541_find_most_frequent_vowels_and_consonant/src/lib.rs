pub struct Solution {}

impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        let c = s
            .as_bytes()
            .iter()
            .fold([0; 26], |mut count, b| {
                count[(b - b'a') as usize] += 1;
                count
            })
            .into_iter()
            .enumerate()
            .fold((0, 0), |max_counts, (i, c)| {
                if [b'a', b'e', b'i', b'o', b'u'].contains(&(i as u8 + b'a')) {
                    (max_counts.0.max(c), max_counts.1)
                } else {
                    (max_counts.0, max_counts.1.max(c))
                }
            });
        c.0 + c.1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3541() {
        assert_eq!(Solution::max_freq_sum("successes".to_string()), 6);
        assert_eq!(Solution::max_freq_sum("aeiaeia".to_string()), 3);
    }
}
