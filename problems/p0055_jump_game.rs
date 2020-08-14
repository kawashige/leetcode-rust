pub struct Solution {}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut i: i32 = nums.len() as i32 - 2;
        while i > -1 {
            if nums[i as usize] != 0 {
                i -= 1;
                continue;
            }

            if i == 0 {
                return false;
            }

            let mut j: i32 = 1;
            loop {
                if nums[(i - j) as usize] > j as i32 {
                    break;
                } else if i - j as i32 == 0 {
                    return false;
                }
                j += 1;
            }
            i = i - j;
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0055() {
        assert!(Solution::can_jump(vec![2, 3, 1, 1, 4]));
        assert!(!Solution::can_jump(vec![3, 2, 1, 0, 4]));
        assert!(Solution::can_jump(vec![0]));
        assert!(!Solution::can_jump(vec![1, 0, 3, 3, 3]));
        assert!(Solution::can_jump(vec![2, 0, 3, 0, 3]));
        assert!(!Solution::can_jump(vec![1, 0, 3, 0, 3]));
        assert!(!Solution::can_jump(vec![0, 1]));
        assert!(Solution::can_jump(vec![1, 0]));
    }
}
