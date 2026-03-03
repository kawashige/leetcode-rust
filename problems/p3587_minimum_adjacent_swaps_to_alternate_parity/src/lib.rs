pub struct Solution {}

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let mut count = [0_i32; 2];
        let mut indices = vec![vec![]; 2];
        for i in 0..nums.len() {
            count[(nums[i] % 2) as usize] += 1;
            indices[(nums[i] % 2) as usize].push(i);
        }
        if 1 < (count[0] - count[1]).abs() {
            return -1;
        }
        if count[0] == count[1] {
            (0..nums.len())
                .step_by(2)
                .map(|i| (i as i32 - indices[0][i / 2] as i32).abs())
                .sum::<i32>()
                .min(
                    (0..nums.len())
                        .step_by(2)
                        .map(|i| (i as i32 - indices[1][i / 2] as i32).abs())
                        .sum(),
                )
        } else if count[0] < count[1] {
            (0..nums.len())
                .step_by(2)
                .map(|i| (i as i32 - indices[1][i / 2] as i32).abs())
                .sum()
        } else {
            (0..nums.len())
                .step_by(2)
                .map(|i| (i as i32 - indices[0][i / 2] as i32).abs())
                .sum()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3587() {
        assert_eq!(Solution::min_swaps(vec![2, 4, 6, 5, 7]), 3);
        assert_eq!(Solution::min_swaps(vec![2, 4, 5, 7]), 1);
        assert_eq!(Solution::min_swaps(vec![1, 2, 3]), 0);
        assert_eq!(Solution::min_swaps(vec![4, 5, 6, 8]), -1);
    }
}
