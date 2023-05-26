use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn recurse(
        i: usize,
        m: usize,
        is_alice: bool,
        piles: &[i32],
        memo: &mut HashMap<(usize, usize, bool), i32>,
    ) -> i32 {
        if let Some(result) = memo.get(&(i, m, is_alice)) {
            return *result;
        }

        let mut alice_piles = Vec::new();
        let mut sum = 0;
        for j in 0..(2 * m).min(piles.len() - i) {
            sum += piles[i + j];
            alice_piles.push(
                if is_alice { sum } else { 0 }
                    + Self::recurse(i + j + 1, m.max(j + 1), !is_alice, piles, memo),
            );
        }

        let result = if is_alice {
            alice_piles.into_iter().max().unwrap_or(0)
        } else {
            alice_piles.into_iter().min().unwrap_or(0)
        };
        memo.insert((i, m, is_alice), result);
        result
    }

    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        Self::recurse(0, 1, true, &piles, &mut HashMap::new())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1140() {
        assert_eq!(Solution::stone_game_ii(vec![2, 7, 9, 4, 4]), 10);
        assert_eq!(Solution::stone_game_ii(vec![1, 2, 3, 4, 5, 100]), 104);
    }
}
