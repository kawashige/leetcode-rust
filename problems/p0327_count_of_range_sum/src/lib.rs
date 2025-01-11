use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let lower = lower as i64;
        let upper = upper as i64;
        let mut result = 0;
        let mut sum = 0;

        let mut lower_heap = BinaryHeap::new();
        let mut upper_heap = BinaryHeap::new();
        lower_heap.push(0);
        upper_heap.push(Reverse(0));
        let mut lower_heap_tmp = BinaryHeap::new();
        let mut upper_heap_tmp = BinaryHeap::new();

        for i in 0..nums.len() {
            sum += nums[i] as i64;

            // lower
            while let Some(x) = lower_heap.pop() {
                if lower <= sum - x {
                    lower_heap.push(x);
                    break;
                }
                lower_heap_tmp.push(Reverse(x));
            }
            while let Some(Reverse(x)) = lower_heap_tmp.pop() {
                if sum - x < lower {
                    lower_heap_tmp.push(Reverse(x));
                    break;
                }
                lower_heap.push(x);
            }

            // upper
            while let Some(Reverse(x)) = upper_heap.pop() {
                if sum - x <= upper {
                    upper_heap.push(Reverse(x));
                    break;
                }
                upper_heap_tmp.push(x);
            }
            while let Some(x) = upper_heap_tmp.pop() {
                if upper < sum - x {
                    upper_heap_tmp.push(x);
                    break;
                }
                upper_heap.push(Reverse(x));
            }

            result += i + 1 - (lower_heap_tmp.len() + upper_heap_tmp.len()).min(i + 1);

            lower_heap.push(sum as i64);
            upper_heap.push(Reverse(sum as i64));
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0327() {
        assert_eq!(
            Solution::count_range_sum(vec![-2147483647, 0, -2147483647, 2147483647], -564, 3864),
            3
        );
        assert_eq!(Solution::count_range_sum(vec![-1, 1], 0, 0), 1);
        assert_eq!(Solution::count_range_sum(vec![0, 0], 0, 0), 3);
        assert_eq!(Solution::count_range_sum(vec![-2, 5, -1], -2, 2), 3);
        assert_eq!(Solution::count_range_sum(vec![0], 0, 0), 1);
    }
}
