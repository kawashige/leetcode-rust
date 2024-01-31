pub struct Solution {}

impl Solution {
    pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        let mut list = vec![vec![]; n as usize];
        for edge in edges {
            list[edge[0] as usize].push(edge[1] as usize);
            list[edge[1] as usize].push(edge[0] as usize);
        }

        let mut seen = vec![false; n as usize];
        for restrict in restricted {
            seen[restrict as usize] = true;
        }
        let mut stack = vec![0];
        let mut count = 0;

        while let Some(node) = stack.pop() {
            if seen[node] {
                continue;
            }
            seen[node] = true;
            count += 1;

            for next in &list[node] {
                stack.push(*next);
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2368() {
        assert_eq!(
            Solution::reachable_nodes(
                7,
                vec![
                    vec![0, 1],
                    vec![1, 2],
                    vec![3, 1],
                    vec![4, 0],
                    vec![0, 5],
                    vec![5, 6]
                ],
                vec![4, 5]
            ),
            4
        );
        assert_eq!(
            Solution::reachable_nodes(
                7,
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![0, 5],
                    vec![0, 4],
                    vec![3, 2],
                    vec![6, 5]
                ],
                vec![4, 1, 2]
            ),
            3
        );
    }
}
