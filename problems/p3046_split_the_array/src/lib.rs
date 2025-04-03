pub struct Solution {}

impl Solution {
    pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
        let mut count = vec![0; 101];
        for n in nums {
            count[n as usize] += 1;
            if count[n as usize] == 3 {
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
    fn test_3046() {
        assert!(Solution::is_possible_to_split(vec![1, 1, 2, 2, 3, 4]));
        assert!(!Solution::is_possible_to_split(vec![1, 1, 1, 1]));
    }
}
