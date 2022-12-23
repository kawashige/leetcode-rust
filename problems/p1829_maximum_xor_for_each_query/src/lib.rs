pub struct Solution {}

impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mut result = vec![0; nums.len()];
        let mut xor = 0;
        let mask = (0..maximum_bit).fold(0, |acc, b| acc | 1 << b);

        for i in 0..nums.len() {
            xor ^= nums[i];
            result[nums.len() - 1 - i] = (xor ^ mask) & mask;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1829() {
        assert_eq!(
            Solution::get_maximum_xor(vec![0, 1, 1, 3], 2),
            vec![0, 3, 2, 3]
        );
        assert_eq!(
            Solution::get_maximum_xor(vec![2, 3, 4, 7], 3),
            vec![5, 2, 6, 5]
        );
        assert_eq!(
            Solution::get_maximum_xor(vec![0, 1, 2, 2, 5, 7], 3),
            vec![4, 3, 6, 4, 6, 7]
        );
    }
}
