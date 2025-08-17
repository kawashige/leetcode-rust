pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut flipped = false;
        let mut result = 0;
        for num in nums {
            if (flipped && num == 1) || (!flipped && num == 0) {
                result += 1;
                flipped = !flipped;
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3192() {
        assert_eq!(Solution::min_operations(vec![0, 1, 1, 0, 1]), 4);
        assert_eq!(Solution::min_operations(vec![1, 0, 0, 0]), 1);
    }
}
