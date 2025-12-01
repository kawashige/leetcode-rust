pub struct Solution {}

impl Solution {
    pub fn recurse(
        cur: usize,
        prev: usize,
        list: &Vec<Vec<usize>>,
        s: &str,
        parent: &mut Vec<i32>,
        char_nodes: &mut Vec<i32>,
    ) {
        if char_nodes[(s.as_bytes()[cur] - b'a') as usize] != -1 {
            parent[cur] = char_nodes[(s.as_bytes()[cur] - b'a') as usize];
        }
        let tmp = char_nodes[(s.as_bytes()[cur] - b'a') as usize];
        char_nodes[(s.as_bytes()[cur] - b'a') as usize] = cur as i32;

        for child in &list[cur] {
            if child == &prev {
                continue;
            }
            Self::recurse(*child, cur, list, s, parent, char_nodes);
        }
        char_nodes[(s.as_bytes()[cur] - b'a') as usize] = tmp;
    }

    pub fn recurse2(cur: usize, prev: usize, list: &Vec<Vec<usize>>, sizes: &mut Vec<i32>) -> i32 {
        let mut size = 1;
        for child in &list[cur] {
            if child == &prev {
                continue;
            }
            size += Self::recurse2(*child, cur, list, sizes);
        }
        sizes[cur] = size;
        size
    }

    pub fn find_subtree_sizes(parent: Vec<i32>, s: String) -> Vec<i32> {
        let mut list = vec![vec![]; parent.len()];
        for i in 1..parent.len() {
            list[parent[i] as usize].push(i);
        }
        let mut parent = parent;
        let mut char_nodes = vec![-1; 26];
        Self::recurse(0, parent.len(), &list, &s, &mut parent, &mut char_nodes);

        let mut list = vec![vec![]; parent.len()];
        for i in 1..parent.len() {
            list[parent[i] as usize].push(i);
        }
        println!("{:?}", parent);
        let mut sizes = vec![0; parent.len()];
        Self::recurse2(0, parent.len(), &list, &mut sizes);
        sizes
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3331() {
        assert_eq!(
            Solution::find_subtree_sizes(vec![-1, 0, 0, 1, 1, 1], "abaabc".to_string()),
            vec![6, 3, 1, 1, 1, 1]
        );
        assert_eq!(
            Solution::find_subtree_sizes(vec![-1, 0, 4, 0, 1], "abbba".to_string()),
            vec![5, 2, 1, 1, 1]
        );
    }
}
