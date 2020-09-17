pub struct Solution {}

impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let directions = [(0, 1), (-1, 0), (0, -1), (1, 0)];
        let (position, direction) = instructions.chars().fold(((0, 0), 0), |(p, d), c| match c {
            'G' => ((p.0 + directions[d].0, p.1 + directions[d].1), d),
            'L' => (p, (d + 1) % 4),
            'R' => (p, (d + 3) % 4),
            _ => unreachable!(),
        });

        position == (0, 0) || direction != 0
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_17() {
        assert!(Solution::is_robot_bounded("GGLLGG".to_string()));
        assert!(!Solution::is_robot_bounded("GG".to_string()));
        assert!(Solution::is_robot_bounded("GL".to_string()));
    }
}
