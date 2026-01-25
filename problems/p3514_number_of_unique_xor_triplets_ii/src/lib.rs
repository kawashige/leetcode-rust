use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        let mut found = vec![false; 2_usize.pow(11)];
        for i in 0..nums.len() {
            for j in 1..i {
                found[(nums[i] ^ nums[j]) as usize] = true;
            }
        }

        let mut set = HashSet::new();
        for i in 0..found.len() {
            if !found[i] {
                continue;
            }
            for j in 0..nums.len() {
                set.insert(nums[j] ^ i as i32);
            }
        }

        set.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3514() {
        assert_eq!(Solution::unique_xor_triplets(vec![1, 3]), 2);
        assert_eq!(Solution::unique_xor_triplets(vec![6, 7, 8, 9]), 4);
    }
}
