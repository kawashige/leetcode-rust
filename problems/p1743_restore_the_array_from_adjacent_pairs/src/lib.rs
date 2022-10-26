use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut result = Vec::with_capacity(adjacent_pairs.len() + 1);

        for pair in adjacent_pairs {
            (*map.entry(pair[0]).or_insert(Vec::with_capacity(2))).push(pair[1]);
            (*map.entry(pair[1]).or_insert(Vec::with_capacity(2))).push(pair[0]);
        }

        let (start, _) = map.iter().find(|(_, v)| v.len() == 1).unwrap();
        let mut cur = *start;

        while result.len() < result.capacity() {
            if map[&cur].len() == 1 || Some(&map[&cur][1]) == result.last() {
                result.push(cur);
                cur = map[&cur][0];
            } else {
                result.push(cur);
                cur = map[&cur][1];
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1743() {
        assert_eq!(
            Solution::restore_array(vec![vec![2, 1], vec![3, 4], vec![3, 2]]),
            vec![4, 3, 2, 1]
        );
        assert_eq!(
            Solution::restore_array(vec![vec![4, -2], vec![1, 4], vec![-3, 1]]),
            vec![-3, 1, 4, -2]
        );
        assert_eq!(
            Solution::restore_array(vec![vec![100000, -100000]]),
            vec![-100000, 100000]
        );
    }
}
