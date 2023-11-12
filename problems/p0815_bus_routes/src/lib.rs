use std::collections::{HashMap, VecDeque};

pub struct Solution {}

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }

        let mut buses = HashMap::new();
        let mut queue = VecDeque::new();
        let mut have_target = vec![false; routes.len()];
        for i in 0..routes.len() {
            for stop in &routes[i] {
                (*buses.entry(*stop as usize).or_insert(vec![])).push(i);
                if stop == &source {
                    queue.push_back((i, 1));
                }
                if stop == &target {
                    have_target[i] = true;
                }
            }
        }
        let mut can_exchange = vec![vec![false; routes.len()]; routes.len()];
        for buses in buses.values() {
            for i in 0..buses.len() {
                for j in i + 1..buses.len() {
                    can_exchange[buses[i]][buses[j]] = true;
                    can_exchange[buses[j]][buses[i]] = true;
                }
            }
        }

        let mut seen = vec![false; routes.len()];
        while let Some((bus, count)) = queue.pop_front() {
            if have_target[bus] {
                return count;
            }
            if seen[bus] {
                continue;
            }
            seen[bus] = true;

            for next_bus in 0..can_exchange.len() {
                if can_exchange[bus][next_bus] {
                    queue.push_back((next_bus, count + 1))
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0815() {
        assert_eq!(
            Solution::num_buses_to_destination(vec![vec![1, 2, 7], vec![3, 6, 7]], 1, 6),
            2
        );
        assert_eq!(
            Solution::num_buses_to_destination(
                vec![
                    vec![7, 12],
                    vec![4, 5, 15],
                    vec![6],
                    vec![15, 19],
                    vec![9, 12, 13]
                ],
                15,
                12
            ),
            -1
        );
    }
}
