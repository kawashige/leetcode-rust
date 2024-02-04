pub struct Solution {}

impl Solution {
    pub fn edge_score(edges: Vec<i32>) -> i32 {
        let mut count = vec![0; edges.len()];
        for i in 0..edges.len() {
            count[edges[i] as usize] += i;
        }
        let mut count = count.into_iter().enumerate().collect::<Vec<_>>();
        count.sort_unstable_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        count[0].0 as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2374() {
        assert_eq!(Solution::edge_score(vec![1, 0, 0, 0, 0, 7, 7, 5]), 7);
        assert_eq!(Solution::edge_score(vec![2, 0, 0, 2]), 0);
    }
}
