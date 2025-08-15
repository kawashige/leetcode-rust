pub struct Solution {}

impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(2);
        let mut seen = vec![false; 100];
        for n in nums {
            if seen[n as usize] {
                result.push(n);
            } else {
                seen[n as usize] = true;
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3289() {
        assert_eq!(Solution::get_sneaky_numbers(vec![0, 1, 1, 0]), vec![1, 0]);
        assert_eq!(
            Solution::get_sneaky_numbers(vec![0, 3, 2, 1, 3, 2]),
            vec![3, 2]
        );
        assert_eq!(
            Solution::get_sneaky_numbers(vec![7, 1, 5, 4, 3, 4, 6, 0, 9, 5, 8, 2]),
            vec![4, 5]
        );
    }
}
