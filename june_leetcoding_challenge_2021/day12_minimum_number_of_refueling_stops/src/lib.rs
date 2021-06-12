use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, mut stations: Vec<Vec<i32>>) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut fuel = start_fuel;
        let mut stop = 0;
        stations.push(vec![target, 0]);

        for i in 0..stations.len() {
            while fuel < stations[i][0] {
                if let Some(x) = heap.pop() {
                    fuel += x;
                    stop += 1;
                } else {
                    return -1;
                }
            }
            heap.push(stations[i][1]);
        }

        stop
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day12() {
        assert_eq!(
            Solution::min_refuel_stops(100, 50, vec![vec![25, 25], vec![50, 50]]),
            1
        );
        assert_eq!(Solution::min_refuel_stops(1, 1, vec![]), 0);
        assert_eq!(Solution::min_refuel_stops(100, 1, vec![vec![10, 100]]), -1);
        assert_eq!(
            Solution::min_refuel_stops(
                100,
                10,
                vec![vec![10, 60], vec![20, 30], vec![30, 30], vec![60, 40]]
            ),
            2
        );
    }
}
