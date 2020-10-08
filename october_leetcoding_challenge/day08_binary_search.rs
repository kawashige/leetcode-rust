pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut i: i32 = 0;
        let mut j: i32 = nums.len() as i32 - 1;
        while i <= j {
            let mid = (i + j) / 2;
            if nums[mid as usize] == target {
                return mid;
            } else if target < nums[mid as usize] {
                j = mid - 1;
            } else {
                i = mid + 1;
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day8() {
        assert_eq!(-1, Solution::search(vec![5], -5));
        assert_eq!(0, Solution::search(vec![5], 5));
        assert_eq!(4, Solution::search(vec![-1, 0, 3, 5, 9, 12], 9));
        assert_eq!(-1, Solution::search(vec![-1, 0, 3, 5, 9, 12], 2));
    }
}
