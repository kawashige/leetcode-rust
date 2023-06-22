pub struct Solution {}

impl Solution {
    pub fn winner_of_game(colors: String) -> bool {
        let mut count = [0; 2];
        let mut consecutive_count = [0; 2];

        for i in 0..colors.len() {
            if colors.as_bytes()[i] == b'A' {
                consecutive_count[0] += 1;
                consecutive_count[1] = 0;
                if 2 < consecutive_count[0] {
                    count[0] += 1;
                }
            } else {
                consecutive_count[1] += 1;
                consecutive_count[0] = 0;
                if 2 < consecutive_count[1] {
                    count[1] += 1;
                }
            }
        }

        count[1] < count[0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2038() {
        assert!(Solution::winner_of_game("AAABABB".to_string()));
        assert!(!Solution::winner_of_game("AA".to_string()));
        assert!(!Solution::winner_of_game("ABBBBBBBAAA".to_string()));
    }
}
