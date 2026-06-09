use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn count_stable_subarrays(capacity: Vec<i32>) -> i64 {
        let mut acc = vec![0; capacity.len() + 1];
        for i in 0..capacity.len() {
            acc[i + 1] = acc[i] + capacity[i] as i64;
        }

        let mut map = HashMap::new();
        let mut result = 0;

        for r in 2..capacity.len() {
            *map.entry((capacity[r - 2], acc[r - 1])).or_insert(0) += 1;
            result += map
                .get(&(capacity[r], acc[r] - capacity[r] as i64))
                .unwrap_or(&0);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3728() {
        assert_eq!(Solution::count_stable_subarrays(vec![9, 3, 3, 3, 9]), 2);
        assert_eq!(Solution::count_stable_subarrays(vec![1, 2, 3, 4, 5]), 0);
        assert_eq!(
            Solution::count_stable_subarrays(vec![-4, 4, 0, 0, -8, -4]),
            1
        );
    }
}

fn main() {}
