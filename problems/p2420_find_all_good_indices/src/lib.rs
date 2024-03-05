pub struct Solution {}

impl Solution {
    pub fn good_indices(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut left_non_increaseing = vec![1; nums.len()];
        let mut right_non_decreasing = vec![1; nums.len()];

        for i in 1..nums.len() {
            if nums[i - 1] >= nums[i] {
                left_non_increaseing[i] = left_non_increaseing[i - 1] + 1;
            }
        }
        for i in (0..nums.len() - 1).rev() {
            if nums[i] <= nums[i + 1] {
                right_non_decreasing[i] = right_non_decreasing[i + 1] + 1;
            }
        }

        println!("{:?}", left_non_increaseing);
        println!("{:?}", right_non_decreasing);

        (k..nums.len() as i32 - k)
            .filter(|i| {
                k <= left_non_increaseing[*i as usize - 1]
                    && k <= right_non_decreasing[*i as usize + 1]
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2420() {
        assert_eq!(
            Solution::good_indices(
                vec![388589, 17165, 726687, 401298, 600033, 537254, 301052, 151069, 399955],
                4
            ),
            vec![]
        );
        assert_eq!(
            Solution::good_indices(vec![2, 1, 1, 1, 3, 4, 1], 2),
            vec![2, 3]
        );
        assert_eq!(Solution::good_indices(vec![2, 1, 1, 2], 2), vec![]);
    }
}
