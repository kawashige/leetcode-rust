pub struct Solution {}

impl Solution {
    pub fn rotate_elements(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut indices = Vec::new();
        for i in 0..nums.len() {
            if 0 <= nums[i] {
                indices.push(i);
            }
        }

        if indices.is_empty() {
            return nums;
        }

        let mut result = nums.clone();
        let k = (k as usize) % indices.len();
        for i in 0..indices.len() {
            result[indices[(i + indices.len() - k) % indices.len()]] = nums[indices[i]];
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3819() {
        assert_eq!(
            Solution::rotate_elements(vec![1, -2, 3, -4], 3),
            vec![3, -2, 1, -4]
        );
        assert_eq!(
            Solution::rotate_elements(vec![-3, -2, 7], 1),
            vec![-3, -2, 7]
        );
        assert_eq!(
            Solution::rotate_elements(vec![5, 4, -9, 6], 2),
            vec![6, 5, -9, 4]
        );
    }
}

fn main() {}
