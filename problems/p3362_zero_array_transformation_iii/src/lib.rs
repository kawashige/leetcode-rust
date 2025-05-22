use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn max_removal(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut queries = queries;
        queries.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));

        let mut dec = vec![0; nums.len() + 1];
        let mut heap = BinaryHeap::new();
        let mut j = 0;
        let mut result = 0;

        for i in 0..nums.len() {
            while j < queries.len() && queries[j][0] <= i as i32 {
                heap.push(queries[j][1] as usize);
                j += 1;
            }
            if 0 < i {
                dec[i] += dec[i - 1];
            }
            if nums[i] == 0 {
                continue;
            }
            while dec[i] < nums[i] && !heap.is_empty() {
                if let Some(x) = heap.pop() {
                    if i <= x {
                        dec[i] += 1;
                        dec[x + 1] -= 1;
                    } else {
                        result += 1;
                    }
                }
            }
            if dec[i] < nums[i] {
                return -1;
            }
        }

        (result + heap.len()) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3362() {
        assert_eq!(
            Solution::max_removal(vec![2, 0, 2], vec![vec![0, 2], vec![0, 2], vec![1, 1]]),
            1
        );
        assert_eq!(
            Solution::max_removal(
                vec![1, 1, 1, 1],
                vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![1, 2]]
            ),
            2
        );
        assert_eq!(
            Solution::max_removal(vec![1, 2, 3, 4], vec![vec![0, 3]]),
            -1
        );
    }
}
