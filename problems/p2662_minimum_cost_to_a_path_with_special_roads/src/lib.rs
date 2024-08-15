use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn minimum_cost(start: Vec<i32>, target: Vec<i32>, special_roads: Vec<Vec<i32>>) -> i32 {
        let mut points = special_roads
            .iter()
            .map(|r| vec![(r[0], r[1]), (r[2], r[3])])
            .flatten()
            .collect::<Vec<_>>();
        points.push((start[0], start[1]));
        points.push((target[0], target[1]));
        let points = points
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let mut special_roads = special_roads;
        special_roads.sort_unstable();
        let special_roads = special_roads
            .into_iter()
            .map(|r| (((r[0], r[1]), (r[2], r[3])), r[4]))
            .rev()
            .collect::<HashMap<_, _>>();

        let mut dist = vec![vec![std::i32::MAX; points.len()]; points.len()];
        for i in 0..points.len() {
            for j in 0..points.len() {
                dist[i][j] = dist[i][j]
                    .min((points[i].0 - points[j].0).abs() + (points[i].1 - points[j].1).abs());
                if let Some(d) = special_roads.get(&((points[i], points[j]))) {
                    dist[i][j] = dist[i][j].min(*d);
                }
            }
        }

        for k in 0..points.len() {
            for i in 0..points.len() {
                for j in 0..points.len() {
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                }
            }
        }

        let s = (0..points.len())
            .find(|i| points[*i] == (start[0], start[1]))
            .unwrap();
        let t = (0..points.len())
            .find(|i| points[*i] == (target[0], target[1]))
            .unwrap();
        dist[s][t]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2662() {
        assert_eq!(
            Solution::minimum_cost(
                vec![1, 1],
                vec![1, 3],
                vec![
                    vec![1, 1, 1, 3, 1],
                    vec![1, 2, 1, 1, 2],
                    vec![1, 1, 1, 3, 4],
                    vec![1, 3, 1, 2, 5],
                    vec![1, 2, 1, 3, 4]
                ]
            ),
            1
        );
        assert_eq!(
            Solution::minimum_cost(
                vec![1, 1],
                vec![4, 5],
                vec![vec![1, 2, 3, 3, 2], vec![3, 4, 4, 5, 1]]
            ),
            5
        );
        assert_eq!(
            Solution::minimum_cost(
                vec![1, 1],
                vec![10, 4],
                vec![
                    vec![4, 2, 1, 1, 3],
                    vec![1, 2, 7, 4, 4],
                    vec![10, 3, 6, 1, 2],
                    vec![6, 1, 1, 2, 3]
                ]
            ),
            8
        );
        assert_eq!(
            Solution::minimum_cost(
                vec![3, 2],
                vec![5, 7],
                vec![
                    vec![5, 7, 3, 2, 1],
                    vec![3, 2, 3, 4, 4],
                    vec![3, 3, 5, 5, 5],
                    vec![3, 4, 5, 6, 6]
                ]
            ),
            7
        );
    }
}
