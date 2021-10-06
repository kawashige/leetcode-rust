use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn convert(i: usize) -> Vec<i32> {
        (0..8)
            .fold(Vec::new(), |mut r, j| {
                r.push(if i & 1 << j > 0 { 1 } else { 0 });
                r
            })
            .into_iter()
            .rev()
            .collect()
    }

    pub fn prison_after_n_days(cells: Vec<i32>, n: i32) -> Vec<i32> {
        let init = (0..8).fold(
            0_usize,
            |s, i| if cells[7 - i] == 1 { s | 1 << i } else { s },
        );
        let mut state = vec![init];
        let mut map = HashMap::new();
        map.insert(init, 0);

        for i in 1..=(n as usize) {
            let prev = state.last().unwrap();
            let current = (1..7).fold(0_usize, |s, i| {
                if (prev & 1 << (i - 1) != 0) == (prev & 1 << (i + 1) != 0) {
                    s | 1 << i
                } else {
                    s
                }
            });

            if let Some(j) = map.get(&current) {
                return Self::convert(state[j + (n as usize - j) % (i - j)]);
            } else {
                map.insert(current, i);
                state.push(current);
            }
        }

        Self::convert(*state.last().unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0957() {
        assert_eq!(
            Solution::prison_after_n_days(vec![0, 1, 0, 1, 1, 0, 0, 1], 7),
            vec![0, 0, 1, 1, 0, 0, 0, 0]
        );
        assert_eq!(
            Solution::prison_after_n_days(vec![1, 0, 0, 1, 0, 0, 1, 0], 1000000000),
            vec![0, 0, 1, 1, 1, 1, 1, 0]
        );
    }
}
