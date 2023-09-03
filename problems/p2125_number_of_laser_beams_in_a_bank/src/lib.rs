pub struct Solution {}

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let bank = bank
            .into_iter()
            .filter(|b| b.as_bytes().iter().any(|b| b == &b'1'))
            .collect::<Vec<_>>();

        let mut previous = 0;
        let mut result = 0;

        for i in 0..bank.len() {
            let current = bank[i].as_bytes().iter().filter(|b| b == &&b'1').count() as i32;
            if 0 < current {
                result += previous * current;
                previous = current;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2125() {
        assert_eq!(
            Solution::number_of_beams(vec![
                "011001".to_string(),
                "000000".to_string(),
                "010100".to_string(),
                "001000".to_string()
            ]),
            8
        );
        assert_eq!(
            Solution::number_of_beams(vec![
                "000".to_string(),
                "111".to_string(),
                "000".to_string()
            ]),
            0
        );
    }
}
