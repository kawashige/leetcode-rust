pub struct Solution {}

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let max = *nums.iter().max().unwrap();
        let indices = (0..nums.len())
            .filter(|i| nums[*i] == max)
            .collect::<Vec<_>>();
        if indices.len() < k as usize {
            return 0;
        }

        let mut count = 0;
        for i in 0..=indices.len() - k as usize {
            count += (if i == 0 {
                indices[i] + 1
            } else {
                indices[i] - indices[i - 1]
            }) as i64
                * (nums.len() - indices[i + k as usize - 1]) as i64;
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2962() {
        assert_eq!(Solution::count_subarrays(vec![1, 3, 2, 3, 3], 2), 6);
        assert_eq!(Solution::count_subarrays(vec![1, 4, 2, 1], 3), 0);
    }
}
