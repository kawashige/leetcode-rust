pub struct Solution {}

impl Solution {
    pub fn max_k_distinct(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable_by_key(|n| -n);

        let mut result = vec![];
        for i in 0..nums.len() {
            if result.last() == Some(&nums[i]) {
                continue;
            }
            result.push(nums[i]);
            if result.len() == k as usize {
                break;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3684() {
        assert_eq!(
            Solution::max_k_distinct(vec![84, 93, 100, 77, 90], 3),
            vec![100, 93, 90]
        );
        assert_eq!(
            Solution::max_k_distinct(vec![84, 93, 100, 77, 93], 3),
            vec![100, 93, 84]
        );
        assert_eq!(
            Solution::max_k_distinct(vec![1, 1, 1, 2, 2, 2], 6),
            vec![2, 1]
        );
    }
}

fn main() {}
