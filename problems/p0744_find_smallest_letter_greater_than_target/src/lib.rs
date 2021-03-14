pub struct Solution {}

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        letters
            .into_iter()
            .min_by_key(|c| (*c as u8 + 25 - target as u8) % 26)
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0744() {
        assert_eq!(
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'a'),
            'c'
        );
        assert_eq!(
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'c'),
            'f'
        );
        assert_eq!(
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'd'),
            'f'
        );
        assert_eq!(
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'g'),
            'j'
        );
        assert_eq!(
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'j'),
            'c'
        );
        assert_eq!(
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'k'),
            'c'
        );
    }
}
