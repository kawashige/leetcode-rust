pub struct Solution {}

impl Solution {
    pub fn score(pins: &[i32]) -> i32 {
        let mut score = 0;
        for i in 0..pins.len() {
            score += pins[i];
            if (0 < i && pins[i - 1] == 10) || (1 < i && pins[i - 2] == 10) {
                score += pins[i];
            }
        }
        score
    }

    pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
        let score1 = Self::score(&player1);
        let score2 = Self::score(&player2);
        if score1 == score2 {
            0
        } else if score1 < score2 {
            2
        } else {
            1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2660() {
        assert_eq!(Solution::is_winner(vec![5, 10, 3, 2], vec![6, 5, 7, 3]), 1);
        assert_eq!(Solution::is_winner(vec![3, 5, 7, 6], vec![8, 10, 10, 2]), 2);
        assert_eq!(Solution::is_winner(vec![2, 3], vec![4, 1]), 0);
        assert_eq!(
            Solution::is_winner(vec![1, 1, 1, 10, 10, 10, 10], vec![10, 10, 10, 10, 1, 1, 1]),
            2
        );
    }
}
