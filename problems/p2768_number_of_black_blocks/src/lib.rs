use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn count_black_blocks(m: i32, n: i32, coordinates: Vec<Vec<i32>>) -> Vec<i64> {
        let mut map = HashMap::new();
        for coodinate in coordinates {
            for (di, dj) in [(-1, -1), (-1, 0), (0, -1), (0, 0)].iter() {
                let (i, j) = (coodinate[0] as i32 + di, coodinate[1] as i32 + dj);
                if !(0..m - 1).contains(&i) || !(0..n - 1).contains(&j) {
                    continue;
                }
                *map.entry((i, j)).or_insert(0) += 1;
            }
        }

        let mut result = vec![0; 5];
        for count in map.values() {
            result[*count as usize] += 1;
        }
        result[0] = (m as i64 - 1) * (n as i64 - 1) - result[1..].iter().sum::<i64>();
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2768() {
        assert_eq!(
            Solution::count_black_blocks(3, 3, vec![vec![0, 0]]),
            vec![3, 1, 0, 0, 0]
        );
        assert_eq!(
            Solution::count_black_blocks(3, 3, vec![vec![0, 0], vec![1, 1], vec![0, 2]]),
            vec![0, 2, 2, 0, 0]
        );
    }
}
