use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut balls = HashMap::new();
        let mut colors = HashMap::new();
        let mut result = Vec::with_capacity(queries.len());

        for i in 0..queries.len() {
            if let Some(c) = balls.get(&queries[i][0]) {
                let c = *c;
                if c != queries[i][1] {
                    balls.insert(queries[i][0], queries[i][1]);
                    if colors[&c] == 1 {
                        colors.remove(&c);
                    } else {
                        *colors.get_mut(&c).unwrap() -= 1;
                    }
                    *colors.entry(queries[i][1]).or_insert(0) += 1
                }
            } else {
                balls.insert(queries[i][0], queries[i][1]);
                *colors.entry(queries[i][1]).or_insert(0) += 1
            }
            result.push(colors.len() as i32);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3160() {
        assert_eq!(
            Solution::query_results(4, vec![vec![1, 4], vec![2, 5], vec![1, 3], vec![3, 4]]),
            vec![1, 2, 2, 3]
        );
        assert_eq!(
            Solution::query_results(
                3,
                vec![vec![0, 1], vec![1, 2], vec![2, 2], vec![3, 4], vec![4, 5]]
            ),
            vec![1, 2, 2, 3, 4]
        );
    }
}
