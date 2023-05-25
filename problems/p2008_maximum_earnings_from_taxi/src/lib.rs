use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn max_taxi_earnings(n: i32, rides: Vec<Vec<i32>>) -> i64 {
        let mut earnings = 0;
        let mut heap = BinaryHeap::new();

        let mut rides = rides;
        rides.sort_unstable();
        rides.push(vec![n, n + 1, 0]);

        for i in 0..rides.len() {
            while let Some((Reverse(t), e)) = heap.pop() {
                if rides[i][0] < t {
                    heap.push((Reverse(t), e));
                    break;
                }
                earnings = earnings.max(e);
            }
            heap.push((
                Reverse(rides[i][1]),
                earnings + (rides[i][1] - rides[i][0] + rides[i][2]) as i64,
            ));
        }

        earnings
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2008() {
        assert_eq!(
            Solution::max_taxi_earnings(5, vec![vec![2, 5, 4], vec![1, 5, 1]]),
            7
        );
        assert_eq!(
            Solution::max_taxi_earnings(
                20,
                vec![
                    vec![1, 6, 1],
                    vec![3, 10, 2],
                    vec![10, 12, 3],
                    vec![11, 12, 2],
                    vec![12, 15, 2],
                    vec![13, 18, 1]
                ]
            ),
            20
        );
    }
}
