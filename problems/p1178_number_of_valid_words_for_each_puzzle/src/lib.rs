pub struct Solution {}

impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        let word_bits = words
            .into_iter()
            .map(|word| {
                word.as_bytes()
                    .into_iter()
                    .fold(0, |bit, b| bit | 1 << (*b as usize - 0x61))
            })
            .collect::<Vec<_>>();

        puzzles
            .into_iter()
            .map(|puzzle| {
                let first_bit = 1 << (puzzle.as_bytes()[0] as usize - 0x61);
                let bit = puzzle
                    .as_bytes()
                    .into_iter()
                    .fold(!0, |bit, b| bit & !(1 << (*b as usize - 0x61)));
                word_bits
                    .iter()
                    .filter(|word| **word & first_bit != 0 && **word & bit == 0)
                    .count() as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1178() {
        assert_eq!(
            Solution::find_num_of_valid_words(
                vec![
                    "aaaa".to_string(),
                    "asas".to_string(),
                    "able".to_string(),
                    "ability".to_string(),
                    "actt".to_string(),
                    "actor".to_string(),
                    "access".to_string()
                ],
                vec![
                    "aboveyz".to_string(),
                    "abrodyz".to_string(),
                    "abslute".to_string(),
                    "absoryz".to_string(),
                    "actresz".to_string(),
                    "gaswxyz".to_string()
                ]
            ),
            vec![1, 1, 3, 2, 4, 0]
        );
        assert_eq!(
            Solution::find_num_of_valid_words(
                vec![
                    "apple".to_string(),
                    "pleas".to_string(),
                    "please".to_string()
                ],
                vec![
                    "aelwxyz".to_string(),
                    "aelpxyz".to_string(),
                    "aelpsxy".to_string(),
                    "saelpxy".to_string(),
                    "xaelpsy".to_string()
                ]
            ),
            vec![0, 1, 3, 2, 0]
        );
    }
}
