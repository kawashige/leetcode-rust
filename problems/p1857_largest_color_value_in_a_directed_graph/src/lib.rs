pub struct Solution {}

impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let mut list = vec![vec![]; colors.len()];
        let mut out_count = vec![0; colors.len()];
        for edge in edges {
            list[edge[1] as usize].push(edge[0] as usize);
            out_count[edge[0] as usize] += 1;
        }

        let mut stack = (0..colors.len())
            .filter(|i| out_count[*i] == 0)
            .collect::<Vec<_>>();
        let mut freq = vec![[0; 26]; colors.len()];
        let mut count = 0;
        let mut max_freq = 0;

        while let Some(i) = stack.pop() {
            count += 1;
            let c = (colors.as_bytes()[i] - b'a') as usize;
            freq[i][c] += 1;
            max_freq = max_freq.max(freq[i][c]);
            for j in &list[i] {
                for k in 0..26 {
                    freq[*j][k] = freq[*j][k].max(freq[i][k]);
                }
                out_count[*j] -= 1;
                if out_count[*j] == 0 {
                    stack.push(*j);
                }
            }
        }

        if count != colors.len() {
            -1
        } else {
            max_freq
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1857() {
        assert_eq!(
            Solution::largest_path_value(
                "abaca".to_string(),
                vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![3, 4]]
            ),
            3
        );
        assert_eq!(
            Solution::largest_path_value("a".to_string(), vec![vec![0, 0]]),
            -1
        );
    }
}
