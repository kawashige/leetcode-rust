use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn recurse(
        i: usize,
        source: &Vec<i32>,
        target: &Vec<i32>,
        list: &Vec<Vec<usize>>,
        seen: &mut Vec<bool>,
        map: &mut HashMap<i32, i32>,
    ) {
        if seen[i] {
            return;
        }
        seen[i] = true;
        *map.entry(source[i]).or_insert(0) += 1;
        *map.entry(target[i]).or_insert(0) -= 1;

        for next in &list[i] {
            Self::recurse(*next, source, target, list, seen, map);
        }
    }

    pub fn minimum_hamming_distance(
        source: Vec<i32>,
        target: Vec<i32>,
        allowed_swaps: Vec<Vec<i32>>,
    ) -> i32 {
        let mut list = vec![vec![]; source.len()];
        for swap in allowed_swaps {
            list[swap[0] as usize].push(swap[1] as usize);
            list[swap[1] as usize].push(swap[0] as usize);
        }

        let mut seen = vec![false; source.len()];
        let mut dist = 0;

        for i in 0..source.len() {
            if seen[i] {
                continue;
            }
            let mut map = HashMap::new();
            Self::recurse(i, &source, &target, &list, &mut seen, &mut map);
            dist += map.values().filter(|v| &&0 < v).sum::<i32>();
        }

        dist
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1722() {
        assert_eq!(
            Solution::minimum_hamming_distance(
                vec![1, 2, 3, 4],
                vec![2, 1, 4, 5],
                vec![vec![0, 1], vec![2, 3]]
            ),
            1
        );
        assert_eq!(
            Solution::minimum_hamming_distance(vec![1, 2, 3, 4], vec![1, 3, 2, 4], vec![]),
            2
        );
        assert_eq!(
            Solution::minimum_hamming_distance(
                vec![5, 1, 2, 4, 3],
                vec![1, 5, 4, 2, 3],
                vec![vec![0, 4], vec![4, 2], vec![1, 3], vec![1, 4]]
            ),
            0
        );
    }
}
