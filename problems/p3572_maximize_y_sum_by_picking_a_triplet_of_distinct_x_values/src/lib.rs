use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn max_sum_distinct_triplet(x: Vec<i32>, y: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for i in 0..x.len() {
            if !map.contains_key(&x[i]) || map.get(&x[i]).unwrap() < &y[i] {
                map.insert(x[i], y[i]);
            }
        }
        let mut values = map.values().collect::<Vec<_>>();
        if values.len() < 3 {
            return -1;
        }
        values.sort_unstable_by(|a, b| b.cmp(&a));
        values.into_iter().take(3).sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3572() {
        assert_eq!(
            Solution::max_sum_distinct_triplet(vec![1, 2, 1, 3, 2], vec![5, 3, 4, 6, 2]),
            14
        );
        assert_eq!(
            Solution::max_sum_distinct_triplet(vec![1, 2, 1, 2], vec![4, 5, 6, 7]),
            -1
        );
    }
}
