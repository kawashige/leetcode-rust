pub struct Solution {}

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut seen = vec![false; 10_001];
        for x in nums {
            if seen[x as usize] {
                return x;
            }
            seen[x as usize] = true;
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_961() {
        assert_eq!(Solution::repeated_n_times(vec![1, 2, 3, 3]), 3);
        assert_eq!(Solution::repeated_n_times(vec![2, 1, 2, 5, 3, 2]), 2);
        assert_eq!(Solution::repeated_n_times(vec![5, 1, 5, 2, 5, 3, 5, 4]), 5);
    }
}
