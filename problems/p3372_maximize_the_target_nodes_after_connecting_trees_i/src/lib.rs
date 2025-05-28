pub struct Solution {}

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut list1 = vec![vec![]; edges1.len() + 1];
        for i in 0..edges1.len() {
            list1[edges1[i][0] as usize].push(edges1[i][1] as usize);
            list1[edges1[i][1] as usize].push(edges1[i][0] as usize);
        }
        let mut list2 = vec![vec![]; edges2.len() + 1];
        for i in 0..edges2.len() {
            list2[edges2[i][0] as usize].push(edges2[i][1] as usize);
            list2[edges2[i][1] as usize].push(edges2[i][0] as usize);
        }

        let mut max = 0;
        for i in 0..list2.len() {
            let mut seen = vec![false; list2.len()];
            let mut stack = vec![(i, 0)];
            let mut count = 0;
            while let Some((node, d)) = stack.pop() {
                if seen[node] || k - 1 < d {
                    continue;
                }
                seen[node] = true;
                count += 1;

                for next in &list2[node] {
                    stack.push((*next, d + 1));
                }
            }
            max = max.max(count);
        }

        let mut result = vec![0; list1.len()];

        for i in 0..list1.len() {
            let mut seen = vec![false; list1.len()];
            let mut stack = vec![(i, 0)];
            let mut count = 0;
            while let Some((node, d)) = stack.pop() {
                if seen[node] || k < d {
                    continue;
                }
                seen[node] = true;
                count += 1;

                for next in &list1[node] {
                    stack.push((*next, d + 1));
                }
            }
            result[i] = count + max;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3372() {
        assert_eq!(
            Solution::max_target_nodes(
                vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]],
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![0, 3],
                    vec![2, 7],
                    vec![1, 4],
                    vec![4, 5],
                    vec![4, 6]
                ],
                2
            ),
            vec![9, 7, 9, 8, 8]
        );
        assert_eq!(
            Solution::max_target_nodes(
                vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4]],
                vec![vec![0, 1], vec![1, 2], vec![2, 3]],
                1
            ),
            vec![6, 3, 3, 3, 3]
        );
    }
}
