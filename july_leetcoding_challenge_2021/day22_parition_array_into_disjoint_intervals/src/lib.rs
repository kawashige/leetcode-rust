pub struct Solution {}

impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let mut right_min = vec![0; nums.len()];
        right_min[nums.len() - 1] = *nums.last().unwrap();
        for i in (0..(nums.len() - 1)).rev() {
            right_min[i] = nums[i].min(right_min[i + 1]);
        }

        let mut max = nums[0];
        let mut i = 1;
        while max > right_min[i] {
            max = max.max(nums[i]);
            i += 1;
        }

        i as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day22() {
        assert_eq!(
            Solution::partition_disjoint(vec![26, 51, 40, 58, 42, 76, 30, 48, 79, 91]),
            1
        );
        assert_eq!(Solution::partition_disjoint(vec![5, 0, 3, 8, 6]), 3);
        assert_eq!(Solution::partition_disjoint(vec![1, 1, 1, 0, 6, 12]), 4);
    }
}
