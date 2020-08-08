pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut i: i32 = 0;
        let mut j: i32 = nums.len() as i32 - 1;

        while i <= j {
            let mid = (i + j) / 2;
            println!("i: {}, j: {}, mid: {}", i, j, mid);
            if target == nums[mid as usize] {
                return mid as i32;
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
        -1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0033() {
        assert_eq!(4, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0));
        assert_eq!(-1, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3));
        assert_eq!(-1, Solution::search(vec![], 5));
        assert_eq!(-1, Solution::search(vec![3, 1], 2));
        assert_eq!(0, Solution::search(vec![5, 1, 3], 5));
        assert_eq!(4, Solution::search(vec![4, 5, 6, 7, 8, 1, 2, 3], 8));
        assert_eq!(2, Solution::search(vec![5, 1, 3], 3));
        assert_eq!(1, Solution::search(vec![5, 1, 2, 3, 4], 1));
    }
}
