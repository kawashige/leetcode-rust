pub struct Solution {}

impl Solution {
    pub fn minimum_replacement(nums: Vec<i32>) -> i64 {
        let mut operations = 0;
        let mut bound = nums[nums.len() - 1];

        for i in (0..nums.len() - 1).rev() {
            let separated_count = (nums[i] + bound - 1) / bound;
            operations += separated_count as i64 - 1;
            bound = (nums[i] % bound)
                + (nums[i] - (nums[i] % bound) * separated_count) / separated_count;
        }

        operations
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2366() {
        assert_eq!(
            Solution::minimum_replacement(vec![
                179, 37, 294, 127, 51, 69, 228, 353, 288, 256, 372, 27, 121, 52, 298, 84, 30, 95,
                107, 393, 194, 274, 314, 321, 278, 97, 255, 42, 65, 26, 163, 295, 286, 15, 270,
                224, 112, 18, 211, 151, 390, 199, 143, 108, 243, 172, 133, 230, 61, 91, 355, 297,
                316, 120, 131, 86, 373, 330, 234, 352, 292, 33, 363, 304, 327, 3, 333, 58, 38, 384,
                197, 216, 8, 7, 123, 245, 341, 388, 371, 370, 101, 152, 53, 110, 168, 178, 396,
                125, 109, 254, 383, 29, 244, 266, 167, 126, 63, 262, 193, 233, 134, 124, 206, 115,
                308, 315, 192, 213, 93, 169, 305, 98, 80, 114, 260
            ]),
            16702
        );
        assert_eq!(
            Solution::minimum_replacement(vec![1, 13, 15, 2, 5, 14, 12, 17]),
            20
        );
        assert_eq!(
            Solution::minimum_replacement(vec![19, 7, 2, 24, 11, 16, 1, 11, 23]),
            73
        );
        assert_eq!(Solution::minimum_replacement(vec![3, 9, 3]), 2);
        assert_eq!(Solution::minimum_replacement(vec![1, 2, 3, 4, 5]), 0);
    }
}
