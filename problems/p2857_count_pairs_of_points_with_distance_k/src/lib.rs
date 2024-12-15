use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn count_pairs(coordinates: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut coordinates = coordinates;
        coordinates.sort_unstable_by_key(|c| c[0]);

        let mut mask = std::i32::MAX;
        for i in 0..32 - k.leading_zeros() {
            mask ^= 1 << i;
        }

        let mut map = HashMap::new();
        for i in 0..coordinates.len() {
            (*map
                .entry((coordinates[i][0] & mask, coordinates[i][1] & mask))
                .or_insert(Vec::new()))
            .push(coordinates[i].clone());
        }

        let mut count = 0;
        for v in map.values() {
            for i in 0..v.len() {
                for j in 0..i {
                    if (v[i][0] ^ v[j][0]) + (v[i][1] ^ v[j][1]) == k {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2857() {
        assert_eq!(
            Solution::count_pairs(vec![vec![1, 2], vec![4, 2], vec![1, 3], vec![5, 2]], 5),
            2
        );
        assert_eq!(
            Solution::count_pairs(
                vec![vec![1, 3], vec![1, 3], vec![1, 3], vec![1, 3], vec![1, 3]],
                0
            ),
            10
        );
    }
}
