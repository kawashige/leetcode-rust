pub struct Solution {}

impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return Default::default();
        }
        let mut nums = nums;
        nums.sort_unstable();

        let mut max = vec![1; nums.len()];
        let mut prev = vec![nums.len(); nums.len()];

        for i in 1..nums.len() {
            for j in 0..i {
                if nums[i] % nums[j] == 0 && max[i] < max[j] + 1 {
                    max[i] = max[j] + 1;
                    prev[i] = j;
                }
            }
        }

        let mut result = Vec::new();
        let mut j = (0..nums.len()).max_by_key(|i| max[*i]).unwrap();
        result.push(nums[j]);
        while prev[j] != nums.len() {
            j = prev[j];
            result.push(nums[j]);
        }
        result
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
