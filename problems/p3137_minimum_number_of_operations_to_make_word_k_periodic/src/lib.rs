use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn minimum_operations_to_make_k_periodic(word: String, k: i32) -> i32 {
        let k = k as usize;
        let mut map = HashMap::new();
        for i in (0..word.len()).step_by(k) {
            *map.entry(&word[i..i + k]).or_insert(0) += 1;
        }
        (word.len() / k - map.values().max().unwrap()) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3137() {
        assert_eq!(
            Solution::minimum_operations_to_make_k_periodic("leetcodeleet".to_string(), 4),
            1
        );
        assert_eq!(
            Solution::minimum_operations_to_make_k_periodic("leetcoleet".to_string(), 2),
            3
        );
    }
}
