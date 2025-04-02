use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn recurse(
        nums: &[i32],
        l: i32,
        r: i32,
        score: i32,
        memo: &mut HashMap<(i32, i32), i32>,
    ) -> i32 {
        if r - l <= 0 {
            return 0;
        }
        if let Some(c) = memo.get(&(l, r)) {
            return *c;
        }
        let mut result = 0;
        if nums[l as usize] + nums[r as usize] == score {
            result = result.max(1 + Self::recurse(&nums, l + 1, r - 1, score, memo))
        }
        if nums[l as usize] + nums[l as usize + 1] == score {
            result = result.max(1 + Self::recurse(&nums, l + 2, r, score, memo))
        }
        if nums[r as usize - 1] + nums[r as usize] == score {
            result = result.max(1 + Self::recurse(&nums, l, r - 2, score, memo))
        }
        memo.insert((l, r), result);
        println!(
            "nums: {:?}, l: {}, r: {}, score: {}. resule: {}",
            nums, l, r, score, result
        );
        result
    }

    pub fn max_operations(nums: Vec<i32>) -> i32 {
        Self::recurse(
            &nums,
            1,
            nums.len() as i32 - 2,
            nums[0] + nums[nums.len() - 1],
            &mut HashMap::new(),
        )
        .max(Self::recurse(
            &nums,
            2,
            nums.len() as i32 - 1,
            nums[0] + nums[1],
            &mut HashMap::new(),
        ))
        .max(Self::recurse(
            &nums,
            0,
            nums.len() as i32 - 3,
            nums[nums.len() - 2] + nums[nums.len() - 1],
            &mut HashMap::new(),
        )) + 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3040() {
        assert_eq!(Solution::max_operations(vec![3, 2, 1, 4, 1]), 2);
        assert_eq!(Solution::max_operations(vec![3, 2, 1, 2, 3, 4]), 3);
        assert_eq!(Solution::max_operations(vec![3, 2, 6, 1, 4]), 2);
    }
}
