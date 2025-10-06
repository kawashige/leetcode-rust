pub struct Solution {}

impl Solution {
    pub fn transform_array(nums: Vec<i32>) -> Vec<i32> {
        let even = nums.iter().filter(|x| **x % 2 == 0).count() as usize;
        std::iter::repeat(0)
            .take(even)
            .chain(std::iter::repeat(1).take(nums.len() - even))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3467() {
        assert_eq!(
            Solution::transform_array(vec![4, 3, 2, 1]),
            vec![0, 0, 1, 1]
        );
        assert_eq!(
            Solution::transform_array(vec![1, 5, 1, 4, 2]),
            vec![0, 0, 1, 1, 1]
        );
    }
}
