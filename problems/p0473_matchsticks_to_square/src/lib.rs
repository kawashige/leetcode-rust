pub struct Solution {}

impl Solution {
    pub fn recurse(i: usize, nums: &Vec<i32>, target: i32, sides: &mut [i32; 4]) -> bool {
        if i == nums.len() {
            return true;
        }

        if target < nums[i] {
            return false;
        }

        for j in 0..sides.len() {
            if sides[j] + nums[i] <= target {
                sides[j] += nums[i];
                if Self::recurse(i + 1, nums, target, sides) {
                    return true;
                }
                sides[j] -= nums[i];
            }
        }
        false
    }

    pub fn makesquare(mut nums: Vec<i32>) -> bool {
        if nums.len() < 4 {
            return false;
        }

        let sum_length: i32 = nums.iter().sum();
        if sum_length % 4 != 0 {
            return false;
        }

        let edge_length = sum_length / 4;
        let mut sides = [0; 4];
        nums.sort_unstable_by(|a, b| b.cmp(&a));
        Self::recurse(0, &nums, edge_length, &mut sides)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0473() {
        assert!(!Solution::makesquare(vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 102
        ]));
        assert!(Solution::makesquare(vec![
            13, 11, 1, 8, 6, 7, 8, 8, 6, 7, 8, 9, 8
        ]));
        assert!(Solution::makesquare(vec![
            5, 5, 5, 5, 4, 4, 4, 4, 3, 3, 3, 3
        ]));
        assert!(Solution::makesquare(vec![1, 1, 2, 2, 2]));
        assert!(!Solution::makesquare(vec![3, 3, 3, 3, 4]));
    }
}
