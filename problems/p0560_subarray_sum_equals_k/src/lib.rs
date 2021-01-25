pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut map: HashMap<i32, i32> = vec![(0, 1)].into_iter().collect();
        let mut sum = 0;
        let mut count = 0;
        for n in nums {
            sum += n;
            if let Some(v) = map.get(&(sum - k)) {
                count += v;
            }
            *map.entry(sum).or_insert(0) += 1;
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0560() {
        assert_eq!(Solution::subarray_sum(vec![-1, -1, 1], 0), 1);
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
        assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
    }
}
