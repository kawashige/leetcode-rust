pub struct Solution {}

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut dist = (0..n).collect::<Vec<_>>();
        let mut result = Vec::with_capacity(queries.len());
        let mut from = vec![vec![]; n as usize];

        for i in 0..queries.len() {
            from[queries[i][1] as usize].push(queries[i][0] as usize);
            dist[queries[i][1] as usize] =
                dist[queries[i][1] as usize].min(dist[queries[i][0] as usize] + 1);
            for j in queries[i][1] as usize..dist.len() {
                dist[j] = dist[j].min(dist[j - 1] + 1);
                for k in &from[j] {
                    dist[j] = dist[j].min(dist[*k] + 1);
                }
            }
            result.push(dist[dist.len() - 1]);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3243() {
        assert_eq!(
            Solution::shortest_distance_after_queries(5, vec![vec![2, 4], vec![0, 2], vec![0, 4]]),
            vec![3, 2, 1]
        );
        assert_eq!(
            Solution::shortest_distance_after_queries(4, vec![vec![0, 3], vec![0, 2]]),
            vec![1, 1]
        );
    }
}
