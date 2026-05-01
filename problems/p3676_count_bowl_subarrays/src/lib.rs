pub struct Solution {}

impl Solution {
    pub fn bowl_subarrays(nums: Vec<i32>) -> i64 {
        let mut result = 0;
        let mut stack = vec![(nums[0], 0)];

        for i in 1..nums.len() {
            while let Some((v, j)) = stack.pop() {
                if nums[i] < v {
                    stack.push((v, j));
                    break;
                }
            }
            if let Some((_, j)) = stack.last() {
                if j + 2 <= i {
                    result += 1;
                }
            }

            stack.push((nums[i], i));
        }

        let mut stack = vec![(nums[nums.len() - 1], nums.len() - 1)];
        for i in (0..nums.len() - 1).rev() {
            while let Some((v, j)) = stack.pop() {
                if nums[i] < v {
                    stack.push((v, j));
                    break;
                }
            }
            if let Some((_, j)) = stack.last() {
                if i + 2 <= *j {
                    result += 1;
                }
            }

            stack.push((nums[i], i));
        }
        result as i64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3676() {
        assert_eq!(Solution::bowl_subarrays(vec![2, 5, 3, 1, 4]), 2);
        assert_eq!(Solution::bowl_subarrays(vec![5, 1, 2, 3, 4]), 3);
        assert_eq!(
            Solution::bowl_subarrays(vec![1000000000, 999999999, 999999998]),
            0
        );
    }
}
