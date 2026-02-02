use std::collections::BTreeSet;

pub struct Solution {}

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
        let k = k as usize;
        let dist = dist as usize;

        let mut min_cost = std::i64::MAX;
        let mut max_set: BTreeSet<(i32, usize)> = BTreeSet::new();
        let mut min_set: BTreeSet<(i32, usize)> = BTreeSet::new();
        let mut sum = 0;

        for i in 1..nums.len() {
            if max_set.len() < k - 1 || nums[i] < max_set.last().unwrap().0 {
                max_set.insert((nums[i], i));
                sum += nums[i] as i64;
                if max_set.len() == k {
                    let (v, j) = max_set.pop_last().unwrap();
                    sum -= v as i64;
                    min_set.insert((v, j));
                }
            } else {
                min_set.insert((nums[i], i));
            }

            if max_set.len() >= k - 1 {
                min_cost = min_cost.min(sum);
            }

            if dist <= i - 1 {
                if max_set.contains(&(nums[i - dist], i - dist)) {
                    max_set.remove(&(nums[i - dist], i - dist));
                    sum -= nums[i - dist] as i64;
                    if let Some((v, j)) = min_set.pop_first() {
                        sum += v as i64;
                        max_set.insert((v, j));
                    }
                } else {
                    min_set.remove(&(nums[i - dist], i - dist));
                }
            }
        }

        min_cost + nums[0] as i64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3013() {
        assert_eq!(Solution::minimum_cost(vec![1, 3, 2, 6, 4, 2], 3, 3), 5);
        assert_eq!(Solution::minimum_cost(vec![10, 1, 2, 2, 2, 1], 4, 3), 15);
        assert_eq!(Solution::minimum_cost(vec![10, 8, 18, 9], 3, 1), 36);
    }
}
