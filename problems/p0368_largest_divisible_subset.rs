pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        fn recurse(nums: &[i32], target: i32, opened: &mut HashMap<i32, Vec<i32>>) -> Vec<i32> {
            if let Some(v) = opened.get(&target) {
                return v.clone();
            }

            let mut results = Vec::new();
            for i in 0..nums.len() {
                if nums.len() - i < results.len() {
                    break;
                }
                if target % nums[i] == 0 {
                    let mut tmp = recurse(&nums[(i + 1)..], nums[i], opened);
                    tmp.push(nums[i]);
                    if results.len() < tmp.len() {
                        results = tmp;
                    }
                }
            }
            results
        }

        let mut nums = nums;
        nums.sort_by_key(|n| -n);

        let mut opened = HashMap::new();
        let mut results = Vec::new();
        for i in 0..nums.len() {
            if nums.len() - i < results.len() {
                break;
            }
            if !opened.contains_key(&nums[i]) {
                let mut tmp = recurse(&nums[(i + 1)..], nums[i], &mut opened);
                tmp.push(nums[i]);
                if results.len() < tmp.len() {
                    results = tmp;
                }
            }
        }
        results
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0368() {
        assert_eq!(
            vec![] as Vec<i32>,
            Solution::largest_divisible_subset(vec![])
        );
        assert_eq!(
            vec![1] as Vec<i32>,
            Solution::largest_divisible_subset(vec![1])
        );
        assert_eq!(
            vec![1, 3],
            Solution::largest_divisible_subset(vec![1, 2, 3])
        );
        assert_eq!(
            vec![1, 2, 4, 8],
            Solution::largest_divisible_subset(vec![1, 2, 4, 8])
        );
        assert_eq!(
            vec![1, 2, 4, 8, 72],
            Solution::largest_divisible_subset(vec![1, 2, 4, 8, 9, 72])
        );
        assert_eq!(
            vec![
                1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536,
                131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216, 33554432,
                67108864, 134217728, 268435456, 536870912, 1073741824
            ],
            Solution::largest_divisible_subset(vec![
                1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536,
                131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216, 33554432,
                67108864, 134217728, 268435456, 536870912, 1073741824
            ])
        );
    }
}
