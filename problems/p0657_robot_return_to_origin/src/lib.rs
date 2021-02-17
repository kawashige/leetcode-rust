use std::unreachable;

pub struct Solution {}

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        moves.chars().fold((0, 0), |(x, y), c| match c {
            'U' => (x, y + 1),
            'D' => (x, y - 1),
            'L' => (x - 1, y),
            'R' => (x + 1, y),
            _ => unreachable!(),
        }) == (0, 0)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0657() {
        assert!(Solution::judge_circle("UD".to_string()));
        assert!(!Solution::judge_circle("LL".to_string()));
        assert!(!Solution::judge_circle("RRDD".to_string()));
        assert!(!Solution::judge_circle("LDRRLRUULR".to_string()));
    }
}
