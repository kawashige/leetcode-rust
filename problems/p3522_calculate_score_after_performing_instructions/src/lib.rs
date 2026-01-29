pub struct Solution {}

impl Solution {
    pub fn calculate_score(instructions: Vec<String>, values: Vec<i32>) -> i64 {
        let mut seen = vec![false; instructions.len()];
        let mut score = 0;
        let mut i = 0;

        while (0..instructions.len() as i32).contains(&i) && !seen[i as usize] {
            seen[i as usize] = true;
            if &instructions[i as usize] == "jump" {
                i += values[i as usize];
            } else {
                score += values[i as usize] as i64;
                i += 1;
            }
        }

        score
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3522() {
        assert_eq!(
            Solution::calculate_score(
                vec![
                    "jump".to_string(),
                    "add".to_string(),
                    "add".to_string(),
                    "jump".to_string(),
                    "add".to_string(),
                    "jump".to_string()
                ],
                vec![2, 1, 3, 1, -2, -3]
            ),
            1
        );
        assert_eq!(
            Solution::calculate_score(
                vec!["jump".to_string(), "add".to_string(), "add".to_string()],
                vec![3, 1, 1]
            ),
            0
        );
        assert_eq!(
            Solution::calculate_score(vec!["jump".to_string()], vec![0]),
            0
        );
    }
}
