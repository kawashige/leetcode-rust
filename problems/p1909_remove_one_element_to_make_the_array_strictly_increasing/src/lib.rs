pub struct Solution {}

impl Solution {
    pub fn can_be_increasing(nums: Vec<i32>) -> bool {
        for i in 0..nums.len() {
            let mut found = true;
            for j in 1..nums.len() {
                if j == i || (j - 1 == i && i == 0) {
                    continue;
                }

                if nums[j] <= nums[j - if j - 1 == i { 2 } else { 1 }] {
                    found = false;
                    break;
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
    fn test_1909() {
        assert!(Solution::can_be_increasing(vec![1, 2, 10, 5, 7]));
        assert!(!Solution::can_be_increasing(vec![2, 3, 1, 2]));
        assert!(!Solution::can_be_increasing(vec![1, 1, 1]));
    }
}
