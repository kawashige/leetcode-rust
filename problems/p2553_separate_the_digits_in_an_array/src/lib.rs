pub struct Solution {}

impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        nums.into_iter()
            .map(|num| {
                num.to_string()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as i32)
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2553() {
        assert_eq!(
            Solution::separate_digits(vec![13, 25, 83, 77]),
            vec![1, 3, 2, 5, 8, 3, 7, 7]
        );
        assert_eq!(
            Solution::separate_digits(vec![7, 1, 3, 9]),
            vec![7, 1, 3, 9]
        );
    }
}
