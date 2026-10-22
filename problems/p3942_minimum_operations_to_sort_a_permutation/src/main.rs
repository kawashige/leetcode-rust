pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let i = (0..nums.len()).find(|i| nums[*i] == 0).unwrap();

        // to right
        if (1..nums.len()).all(|j| nums[(i + j) % nums.len()] > nums[(i + j - 1) % nums.len()]) {
            return i.min(nums.len() - 1 - i + 3) as i32;
        }
        // to left
        if (1..nums.len()).all(|j| {
            nums[(nums.len() + i - j) % nums.len()] > nums[(nums.len() + i + 1 - j) % nums.len()]
        }) {
            return (nums.len() - 1 - i).min((i + 1) % nums.len()) as i32 + 1;
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3942() {
        assert_eq!(Solution::min_operations(vec![0, 2, 1]), 2);
        assert_eq!(Solution::min_operations(vec![1, 0, 2]), 2);
        assert_eq!(Solution::min_operations(vec![2, 0, 1, 3]), -1);
    }
}

fn main() {}
