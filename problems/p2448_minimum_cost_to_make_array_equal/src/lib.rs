pub struct Solution {}

impl Solution {
    pub fn min_cost(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
        let mut num_cost = (0..nums.len())
            .map(|i| (nums[i] as i64, cost[i] as i64))
            .collect::<Vec<_>>();
        num_cost.sort_unstable();

        let mut left_cost = vec![0; num_cost.len()];
        let mut right_cost = vec![0; num_cost.len()];

        let mut sum_cost = num_cost[0].1;
        for i in 1..num_cost.len() {
            left_cost[i] = left_cost[i - 1] + sum_cost * (num_cost[i].0 - num_cost[i - 1].0);
            sum_cost += num_cost[i].1;
        }
        let mut sum_cost = num_cost[num_cost.len() - 1].1;
        for i in (0..num_cost.len() - 1).rev() {
            right_cost[i] = right_cost[i + 1] + sum_cost * (num_cost[i + 1].0 - num_cost[i].0);
            sum_cost += num_cost[i].1;
        }

        (0..num_cost.len())
            .map(|i| left_cost[i] + right_cost[i])
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2448() {
        assert_eq!(
            Solution::min_cost(
                vec![
                    735103, 366367, 132236, 133334, 808160, 113001, 49051, 735598, 686615, 665317,
                    999793, 426087, 587000, 649989, 509946, 743518
                ],
                vec![
                    724182, 447415, 723725, 902336, 600863, 287644, 13836, 665183, 448859, 917248,
                    397790, 898215, 790754, 320604, 468575, 825614
                ]
            ),
            1907611126748
        );
        assert_eq!(Solution::min_cost(vec![1, 3, 5, 2], vec![2, 3, 1, 14]), 8);
        assert_eq!(
            Solution::min_cost(vec![2, 2, 2, 2, 2], vec![4, 2, 8, 1, 3]),
            0
        );
    }
}
