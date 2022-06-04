use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        let mut min = std::i32::MAX;
        let mut tmp_min = std::i32::MAX;
        let mut queue: VecDeque<(usize, i32)> = VecDeque::new();

        let mut sum = 0;
        let mut l = 0;

        for r in 0..arr.len() {
            sum += arr[r];
            while target < sum {
                sum -= arr[l];
                l += 1;
            }

            if target == sum {
                let len = (r - l + 1) as i32;
                queue.push_back((r, len));

                while !queue.is_empty() && queue[0].0 < l {
                    if let Some((_, len)) = queue.pop_front() {
                        tmp_min = tmp_min.min(len);
                    }
                }
                min = min.min(len.saturating_add(tmp_min));
            }
        }

        if min == std::i32::MAX {
            -1
        } else {
            min
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1477() {
        assert_eq!(Solution::min_sum_of_lengths(vec![3, 2, 2, 4, 3], 3), 2);
        assert_eq!(Solution::min_sum_of_lengths(vec![7, 3, 4, 7], 7), 2);
        assert_eq!(
            Solution::min_sum_of_lengths(vec![4, 3, 2, 6, 2, 3, 4], 6),
            -1
        );
    }
}
