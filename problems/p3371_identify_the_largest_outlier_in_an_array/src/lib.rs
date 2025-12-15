pub struct Solution {}

impl Solution {
    pub fn get_largest_outlier(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut count = vec![0; 2001];
        for i in 0..nums.len() {
            sum += nums[i];
            count[(nums[i] + 1000) as usize] += 1;
        }

        let mut result = std::i32::MIN;
        for i in 0..nums.len() {
            let x = sum - nums[i] * 2;
            println!(
                "i: {}, nums[i]: {}, x: {}, count[x +1000]: {}",
                i,
                nums[i],
                x,
                count[(x + 1000) as usize]
            );
            if (0..count.len() as i32).contains(&(x + 1000))
                && count[(x + 1000) as usize] > if x != nums[i] { 0 } else { 1 }
            {
                result = result.max(x);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3371() {
        assert_eq!(Solution::get_largest_outlier(vec![2, 3, 5, 10]), 10);
        assert_eq!(Solution::get_largest_outlier(vec![-2, -1, -3, -6, 4]), 4);
        assert_eq!(Solution::get_largest_outlier(vec![1, 1, 1, 1, 1, 5, 5]), 5);
    }
}
