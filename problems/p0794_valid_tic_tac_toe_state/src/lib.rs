pub struct Solution {}

impl Solution {
    pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
        let (tic, tac): (usize, usize) = board
            .into_iter()
            .flat_map(|s| s.chars().collect::<Vec<char>>())
            .enumerate()
            .fold((0, 0), |(tic, tac), (i, c)| match c {
                'X' => (tic | 1 << i, tac),
                'O' => (tic, tac | 1 << i),
                _ => (tic, tac),
            });

        let end = [
            0b111000000,
            0b000111000,
            0b000000111,
            0b100100100,
            0b010010010,
            0b001001001,
            0b100010001,
            0b001010100,
        ];

        let tic_count = tic.count_ones();
        let tac_count = tac.count_ones();

        let tic_win = end.iter().any(|e| e & tic == *e);
        let tac_win = end.iter().any(|e| e & tac == *e);

        (tic_win && !tac_win && tic_count == tac_count + 1)
            || (!tic_win && tac_win && tic_count == tac_count)
            || (!tic_win && !tac_win && (tic_count == tac_count + 1 || tic_count == tac_count))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0794() {
        assert!(!Solution::valid_tic_tac_toe(vec![
            "O  ".to_string(),
            "   ".to_string(),
            "   ".to_string()
        ]));
        assert!(!Solution::valid_tic_tac_toe(vec![
            "XOX".to_string(),
            " X ".to_string(),
            "   ".to_string()
        ]));
        assert!(!Solution::valid_tic_tac_toe(vec![
            "XXX".to_string(),
            "   ".to_string(),
            "OOO".to_string()
        ]));
        assert!(Solution::valid_tic_tac_toe(vec![
            "XOX".to_string(),
            "O O".to_string(),
            "XOX".to_string()
        ]));
        assert!(!Solution::valid_tic_tac_toe(vec![
            "XXX".to_string(),
            "XOO".to_string(),
            "OO ".to_string()
        ]));
    }
}
