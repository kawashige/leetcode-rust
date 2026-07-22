pub struct Solution {}
use std::cmp::{max, min};

struct SegmentTree {
    n: usize,
    arr: Vec<i32>,
    seg: Vec<i32>,
}

impl SegmentTree {
    fn new(arr: Vec<i32>) -> Self {
        let n = arr.len();
        let seg = vec![0; n << 2];
        let mut st = SegmentTree { n, arr, seg };
        st.build(1, 0, n - 1);
        st
    }

    fn build(&mut self, p: usize, l: usize, r: usize) {
        if l == r {
            self.seg[p] = self.arr[l];
            return;
        }

        let mid = (l + r) >> 1;
        self.build(p << 1, l, mid);
        self.build((p << 1) | 1, mid + 1, r);
        self.seg[p] = max(self.seg[p << 1], self.seg[(p << 1) | 1]);
    }

    fn query_internal(&self, p: usize, l: usize, r: usize, L: usize, R: usize) -> i32 {
        if L <= l && r <= R {
            return self.seg[p];
        }

        let mid = (l + r) >> 1;
        let mut res = 0;
        if L <= mid {
            res = max(res, self.query_internal(p << 1, l, mid, L, R));
        }
        if R > mid {
            res = max(res, self.query_internal((p << 1) | 1, mid + 1, r, L, R));
        }

        res
    }

    fn query(&self, L: usize, R: usize) -> i32 {
        if L > R {
            return 0;
        }

        self.query_internal(1, 0, self.n - 1, L, R)
    }
}

fn lower_bound(list: &[usize], target: usize) -> usize {
    let mut left = 0;
    let mut right = list.len();
    while left < right {
        let mid = left + ((right - left) >> 1);
        if list[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}

fn upper_bound(list: &[usize], target: usize) -> usize {
    let mut left = 0;
    let mut right = list.len();
    while left < right {
        let mid = left + ((right - left) >> 1);
        if list[mid] <= target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}

impl Solution {
    pub fn max_active_sections_after_trade(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = s.len();
        let s_chars: Vec<char> = s.chars().collect();
        let cnt1 = s_chars.iter().filter(|&&c| c == '1').count() as i32;

        let mut zero_blocks: Vec<i32> = Vec::new();
        let mut block_left: Vec<usize> = Vec::new();
        let mut block_right: Vec<usize> = Vec::new();

        let mut i = 0;
        while i < n {
            let st = i;
            while i < n && s_chars[i] == s_chars[st] {
                i += 1;
            }
            if s_chars[st] == '0' {
                zero_blocks.push((i - st) as i32);
                block_left.push(st);
                block_right.push(i - 1);
            }
        }

        let m = zero_blocks.len();
        if m < 2 {
            // continuous 0 blocks less than 2 segments, return the answer directly
            return vec![cnt1; queries.len()];
        }

        let mut tmp_sum: Vec<i32> = vec![0; m - 1];
        for k in 0..m - 1 {
            tmp_sum[k] = zero_blocks[k] + zero_blocks[k + 1];
        }
        let seg = SegmentTree::new(tmp_sum);
        let mut ans: Vec<i32> = Vec::new();

        for q in queries {
            let l = q[0] as usize;
            let r = q[1] as usize;
            let idx = lower_bound(&block_right, l);
            let jdx = upper_bound(&block_left, r).wrapping_sub(1);

            // at most 1 continuous block of 0s within the substring
            if idx > m - 1 || jdx >= m || idx >= jdx {
                ans.push(cnt1);
                continue;
            }
            let first_len = (block_right[idx] - max(block_left[idx], l) + 1) as i32; // actual length of the first consecutive block of 0s in the substring
            let last_len = (min(block_right[jdx], r) - block_left[jdx] + 1) as i32; // actual length of the last consecutive block of 0s in the substring
            let best_gain;
            // exactly 2 consecutive 0 blocks within the substring
            if idx + 1 == jdx {
                best_gain = first_len + last_len;
            } else {
                let val1 = first_len + zero_blocks[idx + 1];
                let val2 = zero_blocks[jdx - 1] + last_len;
                let val3 = seg.query(idx + 1, jdx - 2);
                best_gain = max(max(val1, val2), val3);
            }
            ans.push(cnt1 + best_gain);
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3501() {
        assert_eq!(
            Solution::max_active_sections_after_trade("01".to_string(), vec![vec![0, 1]]),
            vec![1]
        );
        assert_eq!(
            Solution::max_active_sections_after_trade(
                "0100".to_string(),
                vec![vec![0, 3], vec![0, 2], vec![1, 3], vec![2, 3]]
            ),
            vec![4, 3, 1, 1]
        );
        assert_eq!(
            Solution::max_active_sections_after_trade(
                "1000100".to_string(),
                vec![vec![1, 5], vec![0, 6], vec![0, 4]]
            ),
            vec![6, 7, 2]
        );
    }
}

fn main() {}
