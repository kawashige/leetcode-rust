use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn dfs(
        s: &mut String,
        tile: &Vec<char>,
        seen: &mut Vec<bool>,
        sequences: &mut HashSet<String>,
    ) {
        for i in 0..tile.len() {
            if seen[i] {
                continue;
            }

            s.push(tile[i]);
            seen[i] = true;
            if !sequences.contains(s) {
                sequences.insert(s.clone());
                Self::dfs(s, tile, seen, sequences);
            }
            seen[i] = false;
            s.pop();
        }
    }

    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut sequences = HashSet::new();
        Self::dfs(
            &mut String::new(),
            &tiles.chars().collect(),
            &mut vec![false; tiles.len()],
            &mut sequences,
        );
        sequences.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1079() {
        assert_eq!(Solution::num_tile_possibilities("AAB".to_string()), 8);
        assert_eq!(Solution::num_tile_possibilities("AAABBC".to_string()), 188);
        assert_eq!(Solution::num_tile_possibilities("V".to_string()), 1);
    }
}
