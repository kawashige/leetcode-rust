pub struct Solution {}

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i64 {
        let mut nums = nums;
        nums.sort_unstable();
        let mid = nums[nums.len() / 2];

        let mut min_p = std::i32::MAX;
        let mut max_p = std::i32::MAX;

        for i in 1.. {
            let mut s = 1;
            for _ in 1..(i + 1) / 2 {
                s *= 10;
            }
            let mut is_end = false;
            for x in s..s * 10 {
                let chars = x.to_string().chars().collect::<Vec<_>>();
                let p = chars
                    .iter()
                    .chain(chars.iter().take(i - chars.len()).rev())
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap();
                if mid <= p && (mid - p).abs() < (mid - max_p).abs() {
                    max_p = p;
                }
                if mid >= p && (mid - p).abs() < (mid - min_p).abs() {
                    min_p = p;
                }
                if nums[nums.len() - 1] < p {
                    is_end = true;
                    break;
                }
            }
            if is_end {
                break;
            }
        }
        nums.iter()
            .map(|n| (n - min_p).abs() as i64)
            .sum::<i64>()
            .min(nums.iter().map(|n| (n - max_p).abs() as i64).sum::<i64>())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2967() {
        assert_eq!(Solution::minimum_cost(vec![301, 309, 312, 322]), 26);
        assert_eq!(Solution::minimum_cost(vec![1, 2, 3, 4, 5]), 6);
        assert_eq!(Solution::minimum_cost(vec![10, 11, 12, 13, 14, 15]), 11);
        assert_eq!(Solution::minimum_cost(vec![22, 33, 22, 33, 22]), 22);
    }
}
