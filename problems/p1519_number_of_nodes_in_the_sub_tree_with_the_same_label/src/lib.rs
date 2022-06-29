pub struct Solution {}

impl Solution {
    pub fn recurse(
        node: usize,
        parent: usize,
        list: &Vec<Vec<usize>>,
        labels: &[usize],
        result: &mut Vec<i32>,
    ) -> [i32; 26] {
        let mut count = [0; 26];
        count[labels[node]] = 1;

        for child in &list[node] {
            if &parent == child {
                continue;
            }

            let child_count = Self::recurse(*child, node, list, labels, result);
            for i in 0..count.len() {
                count[i] += child_count[i];
            }
        }

        result[node] = count[labels[node]];
        count
    }

    pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
        let mut list = vec![vec![]; n as usize];
        for edge in edges {
            list[edge[0] as usize].push(edge[1] as usize);
            list[edge[1] as usize].push(edge[0] as usize);
        }

        let mut result = vec![0; n as usize];
        let labels = labels
            .as_bytes()
            .iter()
            .map(|b| (b - b'a') as usize)
            .collect::<Vec<_>>();

        Self::recurse(0, n as usize, &list, &labels, &mut result);

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1519() {
        assert_eq!(
            Solution::count_sub_trees(
                7,
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![1, 4],
                    vec![1, 5],
                    vec![2, 3],
                    vec![2, 6]
                ],
                "abaedcd".to_string()
            ),
            vec![2, 1, 1, 1, 1, 1, 1]
        );
        assert_eq!(
            Solution::count_sub_trees(
                4,
                vec![vec![0, 1], vec![1, 2], vec![0, 3]],
                "bbbb".to_string()
            ),
            vec![4, 2, 1, 1]
        );
        assert_eq!(
            Solution::count_sub_trees(
                5,
                vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![0, 4]],
                "aabab".to_string()
            ),
            vec![3, 2, 1, 1, 1]
        );
    }
}
