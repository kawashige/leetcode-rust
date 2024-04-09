pub struct Solution {}

impl Solution {
    pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
        let mut count = 0;

        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] == nums[j] {
                    continue;
                }
                for k in j + 1..nums.len() {
                    if nums[i] == nums[k] || nums[j] == nums[k] {
                        continue;
                    }
                    count += 1;
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2475() {
        assert_eq!(Solution::unequal_triplets(vec![4, 4, 2, 4, 3]), 3);
        assert_eq!(Solution::unequal_triplets(vec![1, 1, 1, 1, 1]), 0);
    }
}
