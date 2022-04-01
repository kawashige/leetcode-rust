use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        let mut parent = vec![-1; n as usize];
        for i in 0..n as usize {
            if left_child[i] != -1 {
                if parent[left_child[i] as usize] != -1 {
                    return false;
                }
                parent[left_child[i] as usize] = i as i32;
            }
            if right_child[i] != -1 {
                if parent[right_child[i] as usize] != -1 {
                    return false;
                }
                parent[right_child[i] as usize] = i as i32;
            }
        }

        let mut seen = vec![false; n as usize];
        if let Some(i) = (0..n as usize).find(|i| parent[*i] == -1) {
            let mut queue = VecDeque::new();
            queue.push_back(i as usize);

            while let Some(node) = queue.pop_front() {
                if seen[node] {
                    return false;
                }
                seen[node] = true;
                if left_child[node] != -1 {
                    queue.push_back(left_child[node] as usize);
                }
                if right_child[node] != -1 {
                    queue.push_back(right_child[node] as usize);
                }
            }
        }

        seen.into_iter().all(|b| b)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1361() {
        assert!(!Solution::validate_binary_tree_nodes(
            4,
            vec![1, 0, 3, -1],
            vec![-1, -1, -1, -1]
        ));
        assert!(Solution::validate_binary_tree_nodes(
            4,
            vec![1, -1, 3, -1],
            vec![2, -1, -1, -1]
        ));
        assert!(!Solution::validate_binary_tree_nodes(
            4,
            vec![1, -1, 3, -1],
            vec![2, 3, -1, -1]
        ));
        assert!(!Solution::validate_binary_tree_nodes(
            2,
            vec![1, 0],
            vec![-1, -1]
        ));
    }
}
