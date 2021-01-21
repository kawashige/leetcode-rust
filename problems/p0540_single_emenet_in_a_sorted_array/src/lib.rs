pub struct Solution {}

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut s = 0;
        let mut e = nums.len() - 1;
        while s <= e {
            let mid = (s + e) / 2;
            let even = (mid - s) % 2 == 0;
            if 0 < mid && nums[mid - 1] == nums[mid] {
                if even {
                    e = mid - 2;
                } else {
                    s = mid + 1;
                }
            } else if mid < nums.len() - 1 && nums[mid] == nums[mid + 1] {
                if even {
                    s = mid + 2;
                } else {
                    e = mid - 1;
                }
            } else {
                return nums[mid];
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0540() {
        assert_eq!(
            2,
            Solution::single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8])
        );
        assert_eq!(
            10,
            Solution::single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11])
        );
        assert_eq!(7, Solution::single_non_duplicate(vec![3, 3, 7]));
        assert_eq!(7, Solution::single_non_duplicate(vec![7]));
    }
}
