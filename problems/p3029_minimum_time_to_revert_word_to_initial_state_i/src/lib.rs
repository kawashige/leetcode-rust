pub struct Solution {}

impl Solution {
    pub fn minimum_time_to_initial_state(word: String, k: i32) -> i32 {
        (1..(word.len() + k as usize - 1) / k as usize)
            .find(|i| word.starts_with(&word[(i * k as usize)..]))
            .unwrap_or((word.len() + k as usize - 1) / k as usize) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3029() {
        assert_eq!(
            Solution::minimum_time_to_initial_state("abacaba".to_string(), 3),
            2
        );
        assert_eq!(
            Solution::minimum_time_to_initial_state("abacaba".to_string(), 4),
            1
        );
        assert_eq!(
            Solution::minimum_time_to_initial_state("abcbabcd".to_string(), 2),
            4
        );
    }
}
