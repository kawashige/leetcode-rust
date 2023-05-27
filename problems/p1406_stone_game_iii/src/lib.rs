use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn recurse(
        i: usize,
        is_alice: bool,
        stone_value: &[i32],
        memo: &mut HashMap<(usize, bool), i32>,
    ) -> i32 {
        if i == stone_value.len() {
            return 0;
        }
        if let Some(result) = memo.get(&(i, is_alice)) {
            return *result;
        }

        let mut score = 0;
        let mut result = if is_alice {
            std::i32::MIN
        } else {
            std::i32::MAX
        };
        for j in 0..3.min(stone_value.len() - i) {
            score += stone_value[i + j];

            if is_alice {
                result = result.max(score + Self::recurse(i + j + 1, !is_alice, stone_value, memo));
            } else {
                result = result.min(Self::recurse(i + j + 1, !is_alice, stone_value, memo));
            }
        }
        memo.insert((i, is_alice), result);

        result
    }

    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let sum = stone_value.iter().sum::<i32>();
        let alice_score = Self::recurse(0, true, &stone_value, &mut HashMap::new());
        if sum - alice_score < alice_score {
            "Alice".to_string()
        } else if sum - alice_score > alice_score {
            "Bob".to_string()
        } else {
            "Tie".to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1406() {
        assert_eq!(
            Solution::stone_game_iii(vec![1, 2, 3, 7]),
            "Bob".to_string()
        );
        assert_eq!(
            Solution::stone_game_iii(vec![1, 2, 3, -9]),
            "Alice".to_string()
        );
        assert_eq!(
            Solution::stone_game_iii(vec![1, 2, 3, 6]),
            "Tie".to_string()
        );
    }
}
