pub struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, v| acc ^ v)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0136() {
        assert_eq!(1, Solution::single_number(vec![2, 2, 1]));
        assert_eq!(4, Solution::single_number(vec![4, 1, 2, 1, 2]));
    }
}
