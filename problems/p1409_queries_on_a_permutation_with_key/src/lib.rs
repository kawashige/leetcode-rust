use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
        let mut p = (1..=m).collect::<VecDeque<_>>();

        queries
            .into_iter()
            .map(|q| {
                let i = (0..p.len()).find(|i| p[*i] == q).unwrap();
                let x = p.remove(i).unwrap();
                p.push_front(x);
                i as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1409() {
        assert_eq!(
            Solution::process_queries(vec![3, 1, 2, 1], 5),
            vec![2, 1, 2, 1]
        );
        assert_eq!(
            Solution::process_queries(vec![4, 1, 2, 2], 4),
            vec![3, 1, 2, 0]
        );
        assert_eq!(
            Solution::process_queries(vec![7, 5, 5, 8, 3], 8),
            vec![6, 5, 0, 7, 5]
        );
    }
}
