use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

pub struct Solution {}

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let mut result = vec![1; rains.len()];
        let mut rain_days = HashMap::new();
        for i in (0..rains.len()).rev() {
            if 0 < rains[i] {
                (*rain_days.entry(rains[i]).or_insert(Vec::new())).push(i);
            }
        }

        let mut dry_target = BinaryHeap::new();
        let mut fulled = HashSet::new();

        for i in 0..rains.len() {
            if 0 < rains[i] {
                if fulled.contains(&rains[i]) {
                    return Default::default();
                }
                fulled.insert(rains[i]);
                result[i] = -1;
                let days = rain_days.get_mut(&rains[i]).unwrap();
                days.pop();
                if !days.is_empty() {
                    dry_target.push((Reverse(*days.last().unwrap()), rains[i]))
                }
            } else {
                if let Some((_, lake)) = dry_target.pop() {
                    result[i] = lake;
                    fulled.remove(&lake);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1488() {
        assert_eq!(
            Solution::avoid_flood(vec![1, 0, 2, 3, 0, 1, 2]),
            vec![-1, 1, -1, -1, 2, -1, -1]
        );
        assert_eq!(
            Solution::avoid_flood(vec![1, 2, 3, 4]),
            vec![-1, -1, -1, -1]
        );
        assert_eq!(
            Solution::avoid_flood(vec![1, 2, 0, 0, 2, 1]),
            vec![-1, -1, 2, 1, -1, -1]
        );
        assert_eq!(Solution::avoid_flood(vec![1, 2, 0, 1, 2]), vec![]);
    }
}
