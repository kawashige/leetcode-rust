use std::collections::{BTreeSet, HashMap};

pub struct Solution {}

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let map = arr
            .iter()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .enumerate()
            .map(|(i, x)| (*x, i as i32))
            .collect::<HashMap<_, _>>();

        arr.into_iter().map(|x| map[&x] + 1).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1331() {
        assert_eq!(
            Solution::array_rank_transform(vec![40, 10, 20, 30]),
            vec![4, 1, 2, 3]
        );
        assert_eq!(
            Solution::array_rank_transform(vec![100, 100, 100]),
            vec![1, 1, 1]
        );
        assert_eq!(
            Solution::array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]),
            vec![5, 3, 4, 2, 8, 6, 7, 1, 3]
        );
    }
}
