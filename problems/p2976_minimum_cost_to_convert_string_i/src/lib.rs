pub struct Solution {}

impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        let mut list = vec![vec![]; 26];
        for i in 0..original.len() {
            list[(original[i] as u8 - b'a') as usize]
                .push(((changed[i] as u8 - b'a') as usize, cost[i] as i64));
        }

        let mut cost = [[std::i64::MAX; 26]; 26];
        for i in 0..26 {
            let mut stack = vec![(i, 0)];
            while let Some((j, c)) = stack.pop() {
                if cost[i][j] <= c {
                    continue;
                }
                cost[i][j] = c;
                for (next, next_c) in &list[j] {
                    if cost[i][*next] <= c + next_c {
                        continue;
                    }
                    stack.push((*next, c + next_c));
                }
            }
        }

        let mut result = 0;
        for i in 0..source.len() {
            if cost[(source.as_bytes()[i] - b'a') as usize][(target.as_bytes()[i] - b'a') as usize]
                == std::i64::MAX
            {
                return -1;
            }
            result += cost[(source.as_bytes()[i] - b'a') as usize]
                [(target.as_bytes()[i] - b'a') as usize];
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2976() {
        assert_eq!(
            Solution::minimum_cost(
                "abcd".to_string(),
                "acbe".to_string(),
                vec!['a', 'b', 'c', 'c', 'e', 'd'],
                vec!['b', 'c', 'b', 'e', 'b', 'e'],
                vec![2, 5, 5, 1, 2, 20]
            ),
            28
        );
        assert_eq!(
            Solution::minimum_cost(
                "aaaa".to_string(),
                "bbbb".to_string(),
                vec!['a', 'c'],
                vec!['c', 'b'],
                vec![1, 2]
            ),
            12
        );
    }
}
