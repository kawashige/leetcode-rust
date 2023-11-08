pub struct Solution {}

impl Solution {
    pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
        let diagonal = (sx - fx).abs().min((sy - fy).abs());
        let shortest_t = diagonal + (sx - fx).abs() - diagonal + (sy - fy).abs() - diagonal;

        if shortest_t == 0 && t == 1 {
            false
        } else {
            shortest_t <= t
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2849() {
        assert!(Solution::is_reachable_at_time(2, 4, 7, 7, 6));
        assert!(!Solution::is_reachable_at_time(3, 1, 7, 3, 3));
        assert!(!Solution::is_reachable_at_time(1, 2, 1, 2, 1));
    }
}
