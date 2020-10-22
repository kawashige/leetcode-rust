pub struct Solution {}

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let l = nums.len() as i32;
        (l * (l + 1)) / 2 - nums.iter().sum::<i32>()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0268() {
        assert_eq!(2, Solution::missing_number(vec![3, 0, 1]));
        assert_eq!(2, Solution::missing_number(vec![0, 1]));
        assert_eq!(8, Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]));
        assert_eq!(1, Solution::missing_number(vec![0]));
    }
}
