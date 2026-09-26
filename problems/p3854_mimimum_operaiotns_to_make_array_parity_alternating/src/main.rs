pub struct Solution {}

impl Solution {
    pub fn make_parity_alternating(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 1 {
            return vec![0, 0];
        }

        let count1 = (0..nums.len())
            .filter(|i| nums[*i].abs() % 2 != (*i as i32) % 2)
            .count() as i32;
        let count2 = (0..nums.len())
            .filter(|i| nums[*i].abs() % 2 != (*i as i32 + 1) % 2)
            .count() as i32;

        if count1 < count2 {
            let mut max = std::i32::MIN;
            let mut min = std::i32::MAX;
            for i in 0..nums.len() {
                if nums[i].abs() % 2 != i as i32 % 2 {
                    max = max.max(nums[i] - 1);
                    min = min.min(nums[i] + 1);
                } else {
                    max = max.max(nums[i]);
                    min = min.min(nums[i]);
                }
            }
            vec![count1, (max - min).max(1)]
        } else if count2 < count1 {
            let mut max = std::i32::MIN;
            let mut min = std::i32::MAX;
            for i in 0..nums.len() {
                if nums[i].abs() % 2 != (i as i32 + 1) % 2 {
                    max = max.max(nums[i] - 1);
                    min = min.min(nums[i] + 1);
                } else {
                    max = max.max(nums[i]);
                    min = min.min(nums[i]);
                }
            }
            vec![count2, (max - min).max(1)]
        } else {
            let mut max = std::i32::MIN;
            let mut min = std::i32::MAX;
            for i in 0..nums.len() {
                if nums[i].abs() % 2 != i as i32 % 2 {
                    max = max.max(nums[i] - 1);
                    min = min.min(nums[i] + 1);
                } else {
                    max = max.max(nums[i]);
                    min = min.min(nums[i]);
                }
            }
            let ans2 = max - min;

            let mut max = std::i32::MIN;
            let mut min = std::i32::MAX;
            for i in 0..nums.len() {
                if nums[i].abs() % 2 != (i as i32 + 1) % 2 {
                    max = max.max(nums[i] - 1);
                    min = min.min(nums[i] + 1);
                } else {
                    max = max.max(nums[i]);
                    min = min.min(nums[i]);
                }
            }
            vec![count1, (max - min).min(ans2).max(1)]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3854() {
        assert_eq!(Solution::make_parity_alternating(vec![-3, -2]), vec![0, 1]);
        assert_eq!(
            Solution::make_parity_alternating(vec![-2, -3, 1, 4]),
            vec![2, 6]
        );
        assert_eq!(
            Solution::make_parity_alternating(vec![0, 2, -2]),
            vec![1, 3]
        );
        assert_eq!(Solution::make_parity_alternating(vec![7]), vec![0, 0]);
    }
}

fn main() {}
