pub struct Solution {}

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return false;
        }
        let mut candidates = Vec::new();
        let mut min = std::i32::MAX;
        for n in nums {
            if candidates.is_empty() || candidates.last().unwrap() < &n {
                candidates.push(n);
                if candidates.len() == 3 {
                    return true;
                }
            } else if candidates.len() == 1 && candidates[0] < n {
                candidates[0] = n;
            } else if candidates.len() == 2 && candidates[0] < n && n < candidates[1] {
                candidates[1] = n;
            } else if n < min {
                min = n;
            } else if min < n {
                candidates = vec![min, n];
                min = std::i32::MAX;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0334() {
        assert!(Solution::increasing_triplet(vec![5, 1, 5, 5, 2, 5, 4]));
        assert!(Solution::increasing_triplet(vec![2, 5, 3, 4, 5]));
        assert!(Solution::increasing_triplet(vec![1, 2, 3, 4, 5]));
        assert!(!Solution::increasing_triplet(vec![5, 4, 3, 2, 1]));
        assert!(Solution::increasing_triplet(vec![4, 5, 1, 2, 3]));
    }
}
