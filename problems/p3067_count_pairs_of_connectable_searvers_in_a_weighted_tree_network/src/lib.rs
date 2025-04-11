pub struct Solution {}

impl Solution {
    pub fn recurse(
        cur: usize,
        prev: usize,
        weight: i32,
        list: &Vec<Vec<(usize, i32)>>,
        signal_speed: i32,
    ) -> i32 {
        let mut result = 0;

        if weight % signal_speed == 0 {
            result += 1;
        }
        for (child, w) in &list[cur] {
            if child == &prev {
                continue;
            }
            result += Self::recurse(*child, cur, weight + w, list, signal_speed);
        }

        result
    }
    pub fn count_pairs_of_connectable_servers(edges: Vec<Vec<i32>>, signal_speed: i32) -> Vec<i32> {
        let mut list = vec![vec![]; edges.len() + 1];
        for edge in edges {
            list[edge[0] as usize].push((edge[1] as usize, edge[2]));
            list[edge[1] as usize].push((edge[0] as usize, edge[2]));
        }

        let mut result = Vec::new();
        for i in 0..list.len() {
            let mut pairs = 0;
            let mut values = Vec::new();
            for (child, w) in &list[i] {
                let count = Self::recurse(*child, i, *w, &list, signal_speed);
                values.push(count);
            }
            for i in 0..values.len() {
                for j in 0..i {
                    pairs += values[i] * values[j];
                }
            }
            result.push(pairs);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3067() {
        assert_eq!(
            Solution::count_pairs_of_connectable_servers(
                vec![
                    vec![0, 1, 1],
                    vec![1, 2, 5],
                    vec![2, 3, 13],
                    vec![3, 4, 9],
                    vec![4, 5, 2]
                ],
                1
            ),
            vec![0, 4, 6, 6, 4, 0]
        );
        assert_eq!(
            Solution::count_pairs_of_connectable_servers(
                vec![
                    vec![0, 6, 3],
                    vec![6, 5, 3],
                    vec![0, 3, 1],
                    vec![3, 2, 7],
                    vec![3, 1, 6],
                    vec![3, 4, 2]
                ],
                3
            ),
            vec![2, 0, 0, 0, 0, 0, 2]
        );
    }
}
