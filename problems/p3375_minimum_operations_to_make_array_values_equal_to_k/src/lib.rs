pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut seen = vec![false; 101];
        for n in nums {
            if n < k {
                return -1;
            }
            seen[n as usize] = true;
        }
        (k as usize + 1..seen.len()).filter(|i| seen[*i]).count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3375() {
        assert_eq!(Solution::min_operations(vec![5, 2, 5, 4, 5], 2), 2);
        assert_eq!(Solution::min_operations(vec![2, 1, 2], 2), -1);
        assert_eq!(Solution::min_operations(vec![9, 7, 5, 3], 1), 4);
    }
}
