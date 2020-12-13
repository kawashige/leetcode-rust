pub struct Solution {}

impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        fn recurse(nums: &Vec<i32>, start: usize, end: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
            if start + 1 == end {
                return 0;
            }
            if 0 < memo[start][end] {
                return memo[start][end];
            }

            let mut max = 0;
            for i in (start + 1)..end {
                let v = nums[start] * nums[i] * nums[end]
                    + recurse(nums, start, i, memo)
                    + recurse(nums, i, end, memo);
                max = std::cmp::max(max, v);
            }
            memo[start][end] = max;
            return max;
        }
        let nums = std::iter::once(1)
            .chain(nums.into_iter())
            .chain(std::iter::once(1))
            .collect::<Vec<i32>>();
        let mut memo = vec![vec![0; nums.len()]; nums.len()];
        recurse(&nums, 0, nums.len() - 1, &mut memo)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day13() {
        assert_eq!(167, Solution::max_coins(vec![3, 1, 5, 8]));
        assert_eq!(832, Solution::max_coins(vec![2, 3, 7, 9, 1, 8, 2]));
        assert_eq!(
            1654,
            Solution::max_coins(vec![7, 9, 8, 0, 7, 1, 3, 5, 5, 2, 3])
        );
    }
}
