pub struct Solution {}

impl Solution {
    pub fn dfs(
        nums: &Vec<i32>,
        i: usize,
        g: usize,
        k: usize,
        sum: i32,
        target: i32,
        seen: &mut Vec<bool>,
    ) -> bool {
        if g == k {
            return true;
        }
        if sum == target {
            return Self::dfs(nums, 0, g + 1, k, 0, target, seen);
        } else if sum > target {
            return false;
        }

        for j in i..nums.len() {
            if seen[j] {
                continue;
            }
            seen[j] = true;
            if Self::dfs(nums, j + 1, g, k, sum + nums[j], target, seen) {
                return true;
            }
            seen[j] = false;
        }

        false
    }

    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let sum: i32 = nums.iter().sum();

        if sum % k != 0 {
            return false;
        }

        let target = sum / k;
        let mut seen = vec![false; nums.len()];
        Self::dfs(&nums, 0, 0, k as usize, 0, target, &mut seen)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0698() {
        assert!(!Solution::can_partition_k_subsets(
            vec![85, 35, 40, 64, 86, 45, 63, 16, 5364, 110, 5653, 97, 95],
            7
        ));
        assert!(Solution::can_partition_k_subsets(
            vec![2, 2, 10, 5, 2, 7, 2, 2, 13],
            3
        ));
        assert!(Solution::can_partition_k_subsets(
            vec![4, 3, 2, 3, 5, 2, 1],
            4
        ));
        assert!(Solution::can_partition_k_subsets(
            vec![10, 10, 10, 7, 7, 7, 7, 7, 7, 6, 6, 6],
            3
        ));
    }
}
