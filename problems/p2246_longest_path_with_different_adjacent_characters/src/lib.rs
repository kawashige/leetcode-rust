pub struct Solution {}

impl Solution {
    pub fn recurse(i: usize, childs: &Vec<Vec<usize>>, s: &str, max_length: &mut i32) -> i32 {
        let mut length = 1;
        let mut childs_length = Vec::new();

        for child in &childs[i] {
            let len = Self::recurse(*child, childs, s, max_length);
            if s.as_bytes()[i] != s.as_bytes()[*child] {
                childs_length.push(len);
            }
        }

        childs_length.sort_unstable();
        let path = childs_length.last().unwrap_or(&0) + 1;
        for _ in 0..2 {
            length += childs_length.pop().unwrap_or(0);
        }
        *max_length = std::cmp::max(*max_length, length);

        path
    }

    pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
        let mut childs = vec![vec![]; parent.len()];
        for i in 0..parent.len() {
            if parent[i] != -1 {
                childs[parent[i] as usize].push(i);
            }
        }

        let mut max_length = 0;
        Self::recurse(0, &childs, &s, &mut max_length);
        max_length
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2246() {
        assert_eq!(
            Solution::longest_path(vec![-1, 0, 0, 1, 1, 2], "abacbe".to_string()),
            3
        );
        assert_eq!(
            Solution::longest_path(vec![-1, 0, 0, 0], "aabc".to_string()),
            3
        );
    }
}
