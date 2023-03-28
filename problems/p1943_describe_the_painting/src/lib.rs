use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn split_painting(segments: Vec<Vec<i32>>) -> Vec<Vec<i64>> {
        let mut map = HashMap::new();
        for segment in segments {
            *map.entry(segment[0]).or_insert(0_i64) += segment[2] as i64;
            *map.entry(segment[1]).or_insert(0_i64) -= segment[2] as i64;
        }
        let mut colors = map.into_iter().collect::<Vec<_>>();
        colors.sort_unstable_by_key(|v| v.0);

        let mut result = Vec::new();
        let mut color = 0;

        for i in 0..colors.len() - 1 {
            color += colors[i].1;
            if 0 < color {
                result.push(vec![colors[i].0 as i64, colors[i + 1].0 as i64, color])
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1943() {
        assert_eq!(
            Solution::split_painting(vec![vec![1, 4, 5], vec![4, 7, 7], vec![1, 7, 9]]),
            vec![vec![1, 4, 14], vec![4, 7, 16]]
        );
        assert_eq!(
            Solution::split_painting(vec![vec![1, 7, 9], vec![6, 8, 15], vec![8, 10, 7]]),
            vec![
                vec![1, 6, 9],
                vec![6, 7, 24],
                vec![7, 8, 15],
                vec![8, 10, 7]
            ]
        );
        assert_eq!(
            Solution::split_painting(vec![
                vec![1, 4, 5],
                vec![1, 4, 7],
                vec![4, 7, 1],
                vec![4, 7, 11]
            ]),
            vec![vec![1, 4, 12], vec![4, 7, 12]]
        );
    }
}
