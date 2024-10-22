pub struct Solution {}

impl Solution {
    pub fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut counts = vec![0; 300_002];
        for num in nums {
            counts[num as usize + 100_000 - k] += 1;
            counts[num as usize + 100_000 + k + 1] -= 1;
        }
        let mut result = 0;
        let mut count = 0;
        for i in 0..counts.len() - 1 {
            count += counts[i];
            result = result.max(count);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2779() {
        assert_eq!(Solution::maximum_beauty(vec![4, 6, 1, 2], 2), 3);
        assert_eq!(Solution::maximum_beauty(vec![1, 1, 1, 1], 10), 4);
    }
}
