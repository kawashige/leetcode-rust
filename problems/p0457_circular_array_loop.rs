pub struct Solution {}

impl Solution {
    pub fn circular_array_loop(mut nums: Vec<i32>) -> bool {
        let mut dummy = 1001;
        for i in 0..nums.len() {
            if 1000 < nums[i] {
                continue;
            }
            let is_forward = 0 < nums[i];
            let mut prev = i;
            let mut next = (i + (nums[i] + nums.len() as i32) as usize) % nums.len();
            nums[prev] = dummy;
            while next != prev && nums[next] <= 1000 && (0 < nums[next]) == is_forward {
                prev = next;
                next = (prev + (nums[prev] + nums.len() as i32) as usize) % nums.len();
                nums[prev] = dummy;
                if nums[next] == dummy && next != prev {
                    return true;
                }
            }
            dummy += 1;
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0457() {
        assert!(!Solution::circular_array_loop(vec![-1, -2, -3, -4, -5]));
        assert!(!Solution::circular_array_loop(vec![1, 2, 2, -1]));
        assert!(Solution::circular_array_loop(vec![-1, -1, -1]));
        assert!(Solution::circular_array_loop(vec![-2, -1, 1, -2, 2]));
        assert!(!Solution::circular_array_loop(vec![2, -1, 1, -2, -2]));
        assert!(Solution::circular_array_loop(vec![2, -1, 1, 2, 2]));
        assert!(!Solution::circular_array_loop(vec![-1, 2]));
        assert!(!Solution::circular_array_loop(vec![-2, 1, -1, -2, -2]));
    }
}
