pub struct Solution {}

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut list = vec![vec![false; n as usize]; n as usize];
        for edge in edges {
            list[edge[0] as usize][edge[1] as usize] = true;
            list[edge[1] as usize][edge[0] as usize] = true;
        }

        let mut seen = vec![false; n as usize];
        let mut result = 0;
        for i in 0..seen.len() {
            if seen[i] {
                continue;
            }
            let mut stack = vec![i];
            let mut nodes = vec![];
            while let Some(node) = stack.pop() {
                if seen[node] {
                    continue;
                }
                seen[node] = true;
                nodes.push(node);
                for j in 0..seen.len() {
                    if !seen[j] && list[node][j] {
                        stack.push(j);
                    }
                }
            }

            let mut found = true;
            for j in 0..nodes.len() {
                for k in j + 1..nodes.len() {
                    if !list[nodes[j]][nodes[k]] {
                        found = false;
                        break;
                    }
                }
                if !found {
                    break;
                }
            }
            if found {
                result += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2685() {
        assert_eq!(
            Solution::count_complete_components(
                6,
                vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4]]
            ),
            3
        );
        assert_eq!(
            Solution::count_complete_components(
                6,
                vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4], vec![3, 5]]
            ),
            1
        );
    }
}
