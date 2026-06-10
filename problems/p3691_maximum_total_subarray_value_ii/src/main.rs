pub struct Solution {}

use std::collections::BinaryHeap;

impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let logn = 32 - (n as u32).leading_zeros() as usize;
        let mut st_max = vec![vec![0; logn]; n];
        let mut st_min = vec![vec![0; logn]; n];
        for i in 0..n {
            st_max[i][0] = nums[i];
            st_min[i][0] = nums[i];
        }
        for j in 1..logn {
            for i in 0..n {
                if i + (1 << j) > n {
                    break;
                }
                st_max[i][j] = st_max[i][j - 1].max(st_max[i + (1 << (j - 1))][j - 1]);
                st_min[i][j] = st_min[i][j - 1].min(st_min[i + (1 << (j - 1))][j - 1]);
            }
        }
        let query_max = |l: usize, r: usize| -> i32 {
            let j = 31 - ((r - l + 1) as u32).leading_zeros() as usize;
            st_max[l][j].max(st_max[r + 1 - (1 << j)][j])
        };
        let query_min = |l: usize, r: usize| -> i32 {
            let j = 31 - ((r - l + 1) as u32).leading_zeros() as usize;
            st_min[l][j].min(st_min[r + 1 - (1 << j)][j])
        };
        let mut heap = BinaryHeap::new();
        for l in 0..n {
            let val = query_max(l, n - 1) - query_min(l, n - 1);
            heap.push((val, l, n - 1));
        }
        let mut ans: i64 = 0;
        let mut k = k as usize;
        while k > 0 {
            if let Some((val, l, r)) = heap.pop() {
                ans += val as i64;
                if r > l {
                    let new_val = query_max(l, r - 1) - query_min(l, r - 1);
                    heap.push((new_val, l, r - 1));
                }
            }
            k -= 1;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3691() {
        assert_eq!(Solution::max_total_value(vec![1, 3, 2], 2), 4);
        assert_eq!(Solution::max_total_value(vec![4, 2, 5, 1], 3), 12);
    }
}

fn main() {}
