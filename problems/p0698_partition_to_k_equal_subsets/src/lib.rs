pub struct Solution {}

impl Solution {
    pub fn dfs(nums: &Vec<i32>, i: usize, remains: &mut Vec<i32>) -> bool {
        if remains.iter().all(|r| r == &0) {
            return true;
        }
        if i > nums.len() - 1 {
            return false;
        }

        for j in 0..remains.len() {
            if nums[i] > remains[j] {
                continue;
            }

            remains[j] -= nums[i];
            if Self::dfs(nums, i + 1, remains) {
                return true;
            }
            remains[j] += nums[i]
        }

        false
    }

    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let sum: i32 = nums.iter().sum();

        if sum % k != 0 {
            return false;
        }
        let target = sum / k;
        let mut remains = vec![target; k as usize];
        if nums[0] > target {
            return false;
        }
        remains[0] -= nums[0];
        Self::dfs(&nums, 1, &mut remains)
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
