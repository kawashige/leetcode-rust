use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        let mut events = events;
        events.sort_unstable();

        let mut max_value = 0;
        let mut result = 0;

        let mut inprogress = BinaryHeap::new();

        for i in 0..events.len() {
            while let Some((Reverse(e), v)) = inprogress.pop() {
                if events[i][0] <= e {
                    inprogress.push((Reverse(e), v));
                    break;
                }
                max_value = max_value.max(v);
            }
            result = result.max(events[i][2] + max_value);
            inprogress.push((Reverse(events[i][1]), events[i][2]));
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2054() {
        assert_eq!(
            Solution::max_two_events(vec![vec![1, 3, 2], vec![4, 5, 2], vec![2, 4, 3]]),
            4
        );
        assert_eq!(
            Solution::max_two_events(vec![vec![1, 3, 2], vec![4, 5, 2], vec![1, 5, 5]]),
            5
        );
        assert_eq!(
            Solution::max_two_events(vec![vec![1, 5, 3], vec![1, 5, 1], vec![6, 6, 5]]),
            8
        );
    }
}
