pub struct Solution {}

impl Solution {
    pub fn dfs(
        i: usize,
        list: &Vec<Vec<usize>>,
        org_chars: &Vec<char>,
        seen: &mut Vec<bool>,
        indices: &mut Vec<usize>,
        chars: &mut Vec<char>,
    ) {
        seen[i] = true;
        indices.push(i);
        chars.push(org_chars[i]);

        for next in &list[i] {
            if seen[*next] {
                continue;
            }

            Self::dfs(*next, list, org_chars, seen, indices, chars);
        }
    }

    pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        let mut list = vec![vec![]; s.len()];
        for pair in pairs {
            list[pair[0] as usize].push(pair[1] as usize);
            list[pair[1] as usize].push(pair[0] as usize);
        }

        let org_chars = s.chars().collect::<Vec<char>>();
        let mut result_chars = vec!['a'; s.len()];
        let mut seen = vec![false; s.len()];

        for i in 0..s.len() {
            if seen[i] {
                continue;
            }

            let mut indices = Vec::new();
            let mut chars = Vec::new();

            Self::dfs(i, &list, &org_chars, &mut seen, &mut indices, &mut chars);

            indices.sort_unstable();
            chars.sort_unstable();

            for (j, c) in indices.into_iter().zip(chars.into_iter()) {
                result_chars[j] = c;
            }
        }

        result_chars.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1202() {
        assert_eq!(
            Solution::smallest_string_with_swaps("dcab".to_string(), vec![vec![0, 3], vec![1, 2]]),
            "bacd".to_string()
        );
        assert_eq!(
            Solution::smallest_string_with_swaps(
                "dcab".to_string(),
                vec![vec![0, 3], vec![1, 2], vec![0, 2]]
            ),
            "abcd".to_string()
        );
        assert_eq!(
            Solution::smallest_string_with_swaps("cba".to_string(), vec![vec![0, 1], vec![1, 2]]),
            "abc".to_string()
        );
    }
}
