pub struct Solution {}

impl Solution {
    pub fn score_balance(s: String) -> bool {
        let total_score = s
            .as_bytes()
            .iter()
            .fold(0, |acc, b| acc + (b - b'a') as usize + 1);
        let mut score = 0;
        for i in 0..s.len() - 1 {
            score += (s.as_bytes()[i] - b'a' + 1) as usize;
            if total_score - score == score {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3707() {
        assert!(Solution::score_balance("adcb".to_string()));
        assert!(!Solution::score_balance("bace".to_string()));
        assert!(!Solution::score_balance("edb".to_string()));
    }
}

fn main() {}
