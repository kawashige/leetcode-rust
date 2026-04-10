pub struct Solution {}

impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        let mut result = std::i32::MAX;

        for i in 0..nums.len() {
            for j in 0..i {
                if nums[i] != nums[j] {
                    continue;
                }
                for k in 0..j {
                    if nums[j] != nums[k] {
                        continue;
                    }
                    result = result.min((i - j + i - k + j - k) as i32);
                }
            }
        }

        if result == std::i32::MAX { -1 } else { result }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3740() {
        assert_eq!(Solution::minimum_distance(vec![1, 2, 1, 1, 3]), 6);
        assert_eq!(Solution::minimum_distance(vec![1, 1, 2, 3, 2, 1, 2]), 8);
        assert_eq!(Solution::minimum_distance(vec![1]), -1);
    }
}
