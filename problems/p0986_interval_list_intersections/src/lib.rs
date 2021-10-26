use std::collections::BTreeMap;

pub struct Solution {}

impl Solution {
    pub fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let mut r = Vec::new();

        let mut map = BTreeMap::new();
        for i in 0..first_list.len() {
            *map.entry(first_list[i][0]).or_insert(0) += 1;
            *map.entry(first_list[i][1]).or_insert(0) -= 1;
        }
        for i in 0..second_list.len() {
            *map.entry(second_list[i][0]).or_insert(0) += 1;
            *map.entry(second_list[i][1]).or_insert(0) -= 1;
        }

        let mut c = 0;
        let mut interval = Vec::new();
        for (k, v) in map {
            match (c, v) {
                (0, 2) | (1, 1) => {
                    interval.push(k);
                }
                (2, -1) | (2, -2) => {
                    interval.push(k);
                    r.push(interval);
                    interval = Vec::new();
                }
                (1, 0) => {
                    r.push(vec![k, k]);
                }
                _ => {}
            }
            c += v;
        }

        r
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0986() {
        assert_eq!(
            Solution::interval_intersection(
                vec![vec![0, 2], vec![5, 10], vec![13, 23], vec![24, 25]],
                vec![vec![1, 5], vec![8, 12], vec![15, 24], vec![25, 26]]
            ),
            vec![
                vec![1, 2],
                vec![5, 5],
                vec![8, 10],
                vec![15, 23],
                vec![24, 24],
                vec![25, 25]
            ]
        );
        assert_eq!(
            Solution::interval_intersection(vec![vec![1, 3], vec![5, 9]], vec![]),
            vec![] as Vec<Vec<i32>>
        );
        assert_eq!(
            Solution::interval_intersection(vec![], vec![vec![4, 8], vec![10, 12]]),
            vec![] as Vec<Vec<i32>>
        );
        assert_eq!(
            Solution::interval_intersection(vec![vec![1, 7]], vec![vec![3, 10]]),
            vec![vec![3, 7]]
        );
    }
}
