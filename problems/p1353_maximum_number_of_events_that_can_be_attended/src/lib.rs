use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn max_events(mut events: Vec<Vec<i32>>) -> i32 {
        events.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));

        let mut heap = BinaryHeap::new();
        let mut count = 0;
        let mut i = 0;

        for t in 1..=100_000 {
            while i < events.len() && events[i][0] <= t {
                heap.push(Reverse(events[i][1]));
                i += 1;
            }

            if let Some(Reverse(e)) = heap.pop() {
                if t <= e {
                    count += 1;
                }
            }

            while let Some(Reverse(e)) = heap.peek() {
                if &t < e {
                    break;
                }
                heap.pop();
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1353() {
        assert_eq!(
            Solution::max_events(vec![vec![1, 2], vec![2, 3], vec![3, 4]]),
            3
        );
        assert_eq!(
            Solution::max_events(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 2]]),
            4
        );
    }
}
