pub struct Solution {}

impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() == 2 {
            return true;
        }
        let k = k as usize;
        for i in 0..nums.len() - 2 * k + 1 {
            let mut found = true;
            for j in i + 1..i + k {
                if nums[j - 1] >= nums[j] {
                    found = false;
                    break;
                }
            }
            if found {
                for j in i + k + 1..i + k * 2 {
                    if nums[j - 1] >= nums[j] {
                        found = false;
                        break;
                    }
                }
            }
            if found {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3349() {
        assert!(Solution::has_increasing_subarrays(
            vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1],
            3
        ));
        assert!(!Solution::has_increasing_subarrays(
            vec![1, 2, 3, 4, 4, 4, 4, 5, 6, 7],
            5
        ));
    }
}
