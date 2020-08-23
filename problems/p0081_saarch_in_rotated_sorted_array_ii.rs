pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let mut i: i32 = 0;
        let mut j: i32 = nums.len() as i32 - 1;

        while i <= j {
            while i < nums.len() as i32 - 1 && nums[i as usize] == nums[i as usize + 1] {
                i += 1;
            }
            while i < j
                && (nums[j as usize] == nums[j as usize - 1]
                    || nums[i as usize] == nums[j as usize])
            {
                j -= 1;
            }
            let mid = (i + j) / 2;
            println!("i: {}, j: {}, mid: {}", i, j, mid);
            if target == nums[mid as usize] {
                return true;
            } else if nums[i as usize] <= nums[mid as usize] {
                if nums[i as usize] <= target && target < nums[mid as usize] {
                    j = mid - 1;
                } else {
                    i = mid + 1;
                }
            } else {
                if nums[mid as usize] < target && target <= nums[j as usize] {
                    i = mid + 1;
                } else {
                    j = mid - 1;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0081() {
        assert!(Solution::search(vec![1, 5, 6, 0, 0, 1, 2], 0));
        assert!(!Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3));
        assert!(Solution::search(vec![1, 3, 1, 1, 3], 3));
        assert!(Solution::search(vec![1, 1, 1, 1, 3], 3));
        assert!(Solution::search(vec![1, 3, 5], 1));
        assert!(Solution::search(vec![1, 3, 5], 3));
        assert!(Solution::search(vec![3, 5, 1], 3));
        assert!(Solution::search(vec![1, 1, 3, 1], 3));
        assert!(Solution::search(vec![0, 0, 1, 1, 2, 0], 2));
    }
}
