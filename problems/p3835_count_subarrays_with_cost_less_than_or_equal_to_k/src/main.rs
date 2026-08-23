use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let mut max_q = VecDeque::new();
        let mut min_q = VecDeque::new();

        let mut result = 0;
        let mut l = 0;
        for r in 0..nums.len() {
            while let Some(&i) = max_q.back() {
                if nums[i] <= nums[r] {
                    max_q.pop_back();
                } else {
                    break;
                }
            }
            max_q.push_back(r);

            while let Some(&i) = min_q.back() {
                if nums[i] >= nums[r] {
                    min_q.pop_back();
                } else {
                    break;
                }
            }
            min_q.push_back(r);

            while k
                < (nums[*max_q.front().unwrap()] - nums[*min_q.front().unwrap()]) as i64
                    * (r - l + 1) as i64
            {
                while let Some(&i) = max_q.front() {
                    if i <= l {
                        max_q.pop_front();
                    } else {
                        break;
                    }
                }
                while let Some(&i) = min_q.front() {
                    if i <= l {
                        min_q.pop_front();
                    } else {
                        break;
                    }
                }
                l += 1;
            }
            result += (r - l + 1) as i64;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3835() {
        assert_eq!(Solution::count_subarrays(vec![1, 3, 2], 4), 5);
        assert_eq!(Solution::count_subarrays(vec![5, 5, 5, 5], 0), 10);
        assert_eq!(Solution::count_subarrays(vec![1, 2, 3], 0), 3);
    }
}

fn main() {}
