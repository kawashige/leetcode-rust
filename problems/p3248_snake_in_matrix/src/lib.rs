pub struct Solution {}

impl Solution {
    pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
        let mut pos = (0, 0);
        for c in &commands {
            let d = match c.as_str() {
                "UP" => (-1, 0),
                "DOWN" => (1, 0),
                "RIGHT" => (0, 1),
                "LEFT" => (0, -1),
                _ => unreachable!(),
            };
            pos.0 += d.0;
            pos.1 += d.1;
        }
        pos.0 * n + pos.1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3248() {
        assert_eq!(
            Solution::final_position_of_snake(2, vec!["RIGHT".to_string(), "DOWN".to_string()]),
            3
        );
        assert_eq!(
            Solution::final_position_of_snake(
                3,
                vec!["DOWN".to_string(), "RIGHT".to_string(), "UP".to_string()]
            ),
            1
        );
    }
}
