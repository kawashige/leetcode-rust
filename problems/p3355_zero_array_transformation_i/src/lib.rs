pub struct Solution {}

impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let mut dec = vec![0; nums.len() + 1];
        for q in queries {
            dec[q[0] as usize] += 1;
            dec[q[1] as usize + 1] -= 1;
        }

        for i in 0..nums.len() {
            if 0 < i {
                dec[i] += dec[i - 1];
            }
            if 0 < nums[i] - dec[i] {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3355() {
        assert!(Solution::is_zero_array(vec![1, 0, 1], vec![vec![0, 2]]));
        assert!(!Solution::is_zero_array(
            vec![4, 3, 2, 1],
            vec![vec![1, 3], vec![0, 2]]
        ));
    }
}
