pub struct Solution {}

impl Solution {
    pub fn count_elements(nums: Vec<i32>) -> i32 {
        let min = *nums.iter().min().unwrap();
        let max = *nums.iter().max().unwrap();
        nums.into_iter()
            .filter(|num| (min + 1..max).contains(num))
            .count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2148() {
        assert_eq!(Solution::count_elements(vec![11, 7, 2, 15]), 2);
        assert_eq!(Solution::count_elements(vec![-3, 3, 3, 90]), 2);
    }
}
