use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        let mut left = vec![-1; nums.len()];
        let mut right = vec![-1; nums.len()];
        let mut heap = BinaryHeap::new();
        let mut removed = vec![false; nums.len()];
        let mut nums = nums.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let mut remains = 0;

        for i in 0..nums.len() {
            if 0 < i {
                if nums[i - 1] > nums[i] {
                    remains += 1;
                }
                heap.push((Reverse(nums[i] + nums[i - 1]), Reverse(i - 1), i));
                left[i] = i as i32 - 1;
            }
            if i < nums.len() - 1 {
                right[i] = i as i32 + 1;
            }
        }

        let mut count = 0;
        while 0 < remains {
            if let Some((Reverse(sum), Reverse(i), j)) = heap.pop() {
                if removed[i] || removed[j] || nums[i] + nums[j] != sum {
                    continue;
                }
                count += 1;
                if nums[i] > nums[j] {
                    remains -= 1;
                }
                right[i] = right[j];
                if right[j] != -1 {
                    left[right[j] as usize] = i as i32;
                }
                if left[i] != -1 {
                    if nums[left[i] as usize] > nums[i] && nums[left[i] as usize] <= sum {
                        remains -= 1;
                    } else if nums[left[i] as usize] <= nums[i] && nums[left[i] as usize] > sum {
                        remains += 1;
                    }
                    heap.push((
                        Reverse(nums[left[i] as usize] + sum),
                        Reverse(left[i] as usize),
                        i,
                    ));
                }
                if right[j] != -1 {
                    if nums[j] > nums[right[j] as usize] && sum <= nums[right[j] as usize] {
                        remains -= 1;
                    } else if nums[j] <= nums[right[j] as usize] && sum > nums[right[j] as usize] {
                        remains += 1;
                    }
                    heap.push((
                        Reverse(nums[right[j] as usize] + sum),
                        Reverse(i),
                        right[j] as usize,
                    ));
                }
                nums[i] = sum;
                removed[j] = true;
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3510() {
        assert_eq!(
            Solution::minimum_pair_removal(vec![3, 6, 4, -6, 2, -4, 5, -7, -3, 6, 3, -4]),
            10
        );
        assert_eq!(Solution::minimum_pair_removal(vec![5, 2, 3, 1]), 2);
        assert_eq!(Solution::minimum_pair_removal(vec![1, 2, 2]), 0);
    }
}
