pub struct Solution {}

impl Solution {
    pub fn max_sum_div_three(mut nums: Vec<i32>) -> i32 {
        let mut r1 = Vec::with_capacity(2);
        let mut r2 = Vec::with_capacity(2);
        let mut sum = 0;

        nums.sort_unstable();

        for n in nums {
            sum += n;
            match n % 3 {
                1 if r1.len() < 2 => r1.push(n),
                2 if r2.len() < 2 => r2.push(n),
                _ => {}
            }
        }

        match sum % 3 {
            0 => sum,
            1 if !r1.is_empty() && r2.len() == 2 => sum - r1[0].min(r2[0] + r2[1]),
            1 if !r1.is_empty() => sum - r1[0],
            1 if r2.len() == 2 => sum - (r2[0] + r2[1]),
            2 if !r2.is_empty() && r1.len() == 2 => sum - r2[0].min(r1[0] + r1[1]),
            2 if !r2.is_empty() => sum - r2[0],
            2 if r1.len() == 2 => sum - (r1[0] + r1[1]),
            _ => 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1262() {
        assert_eq!(Solution::max_sum_div_three(vec![2, 6, 2, 2, 7]), 15);
        assert_eq!(Solution::max_sum_div_three(vec![3, 6, 5, 1, 8]), 18);
        assert_eq!(Solution::max_sum_div_three(vec![4]), 0);
        assert_eq!(Solution::max_sum_div_three(vec![1, 2, 3, 4, 4]), 12);
    }
}
