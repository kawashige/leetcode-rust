use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        let mut count = nums
            .into_iter()
            .fold(HashMap::new(), |mut count, num| {
                if num % 2 == 0 {
                    *count.entry(num).or_insert(0) += 1;
                }
                count
            })
            .into_iter()
            .collect::<Vec<_>>();
        if count.is_empty() {
            return -1;
        }
        count.sort_unstable_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        count[0].0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2404() {
        assert_eq!(Solution::most_frequent_even(vec![0, 1, 2, 2, 4, 4, 1]), 2);
        assert_eq!(Solution::most_frequent_even(vec![4, 4, 4, 9, 2, 4]), 4);
        assert_eq!(
            Solution::most_frequent_even(vec![29, 47, 21, 41, 13, 37, 25, 7]),
            -1
        );
    }
}
