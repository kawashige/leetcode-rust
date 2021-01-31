use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold((HashMap::new(), 0), |(mut count, max), n| {
                *count.entry(n).or_insert(0) += 1;
                let mut tmp = 0;
                if count.contains_key(&(n - 1)) {
                    tmp = count[&n] + count[&(n - 1)];
                }
                if count.contains_key(&(n + 1)) {
                    tmp = std::cmp::max(tmp, count[&n] + count[&(n + 1)]);
                }
                (count, std::cmp::max(max, tmp))
            })
            .1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0954() {
        assert_eq!(Solution::find_lhs(vec![1, 3, 2, 2, 5, 2, 3, 7]), 5);
        assert_eq!(Solution::find_lhs(vec![1, 2, 3, 4]), 2);
        assert_eq!(Solution::find_lhs(vec![1, 1, 1, 1]), 0);
    }
}
