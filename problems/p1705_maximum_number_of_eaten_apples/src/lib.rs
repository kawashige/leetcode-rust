use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut heap = BinaryHeap::new();

        for i in 0..apples.len() {
            heap.push((Reverse(i + days[i] as usize), apples[i]));

            while let Some((Reverse(day), apple)) = heap.pop() {
                if i < day {
                    count += 1;
                    if 1 < apple {
                        heap.push((Reverse(day), apple - 1));
                    }
                    break;
                }
            }
        }

        let mut j = apples.len();
        while !heap.is_empty() {
            while let Some((Reverse(day), apple)) = heap.pop() {
                if j < day {
                    count += 1;
                    if 1 < apple {
                        heap.push((Reverse(day), apple - 1));
                    }
                    break;
                }
            }
            j += 1;
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1705() {
        assert_eq!(
            Solution::eaten_apples(vec![1, 2, 3, 5, 2], vec![3, 2, 1, 4, 2]),
            7
        );
        assert_eq!(
            Solution::eaten_apples(vec![3, 0, 0, 0, 0, 2], vec![3, 0, 0, 0, 0, 2]),
            5
        )
    }
}
