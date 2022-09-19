pub struct Solution {}

impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let mut acc = vec![0; nums.len()];
        acc[0] = nums[0];
        for i in 1..nums.len() {
            acc[i] += acc[i - 1] + nums[i];
        }

        (0..nums.len())
            .map(|i| {
                let mut diff = 0;
                if 0 < i {
                    diff += nums[i] * i as i32 - acc[i - 1];
                }
                if i < acc.len() - 1 {
                    diff +=
                        acc[acc.len() - 1] - acc[i] - nums[i] * (acc.len() as i32 - (i as i32 + 1));
                }
                diff
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1685() {
        assert_eq!(
            Solution::get_sum_absolute_differences(vec![2, 3, 5]),
            vec![4, 3, 5]
        );
        assert_eq!(
            Solution::get_sum_absolute_differences(vec![1, 4, 6, 8, 10]),
            vec![24, 15, 13, 15, 21]
        );
    }
}
