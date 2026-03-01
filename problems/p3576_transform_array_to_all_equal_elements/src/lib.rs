pub struct Solution {}

impl Solution {
    pub fn can_make_equal(nums: Vec<i32>, k: i32) -> bool {
        for x in vec![1, -1] {
            let mut remains = k;
            let mut inv = 1;
            let mut is_ok = true;

            for i in (1..nums.len()).rev() {
                if nums[i] * inv == -x {
                    if remains == 0 {
                        is_ok = false;
                        break;
                    }
                    inv = -1;
                    remains -= 1;
                } else {
                    inv = 1;
                }
            }
            if is_ok && nums[0] * inv == x {
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
    fn test_3576() {
        assert!(Solution::can_make_equal(vec![1, -1, 1, -1, 1], 3));
        assert!(!Solution::can_make_equal(vec![-1, -1, -1, 1, 1, 1], 5));
    }
}
