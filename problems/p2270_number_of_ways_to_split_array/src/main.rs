pub struct Solution {}

impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let mut sum: i64 = nums.iter().map(|num| *num as i64).sum();
        let mut split_sum = 0;
        let mut count = 0;

        for i in 0..nums.len() - 1 {
            split_sum += nums[i] as i64;
            sum -= nums[i] as i64;
            if sum <= split_sum {
                count += 1;
            }
        }

        coun
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2270() {
        assert_eq!(Solution::ways_to_split_array(vec![10, 4, -8, 7]), 2);
        assert_eq!(Solution::ways_to_split_array(vec![2, 3, 1, 0]), 2)
    }
}
