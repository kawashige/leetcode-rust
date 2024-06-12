pub struct Solution {}

impl Solution {
    pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
        let mut result = 0;
        for i in 0..nums.len() / 2 {
            result += format!(
                "{}{}",
                nums[i].to_string(),
                nums[nums.len() - 1 - i].to_string()
            )
            .parse::<i64>()
            .unwrap();
        }
        if nums.len() % 2 == 1 {
            result += nums[nums.len() / 2] as i64;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2562() {
        assert_eq!(Solution::find_the_array_conc_val(vec![7, 52, 2, 4]), 596);
        assert_eq!(
            Solution::find_the_array_conc_val(vec![5, 14, 13, 8, 12]),
            673
        );
    }
}
