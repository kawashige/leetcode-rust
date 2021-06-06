use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set = nums.into_iter().collect::<HashSet<i32>>();
        let mut max = 0;
        for n in &set {
            if !set.contains(&(n - 1)) {
                let mut num = *n;
                let mut count = 1;

                while set.contains(&(num + 1)) {
                    num += 1;
                    count += 1;
                }
                max = std::cmp::max(max, count);
            }
        }
        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day06() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );
    }
}
