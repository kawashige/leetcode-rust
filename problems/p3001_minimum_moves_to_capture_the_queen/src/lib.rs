pub struct Solution {}

impl Solution {
    pub fn min_moves_to_capture_the_queen(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
        if (a == e && (c != a || d < b.min(f) || b.max(f) < d))
            || (b == f && (d != b || c < a.min(e) || a.max(e) < c))
            || (c + d == e + f && (a + b != c + d || a < c.min(e) || c.max(e) < a))
            || (c - d == e - f && (a - b != c - d || a < c.min(e) || c.max(e) < a))
        {
            1
        } else {
            2
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3001() {
        assert_eq!(
            Solution::min_moves_to_capture_the_queen(1, 1, 8, 8, 2, 3),
            2
        );
        assert_eq!(
            Solution::min_moves_to_capture_the_queen(5, 3, 3, 4, 5, 2),
            1
        );
    }
}
