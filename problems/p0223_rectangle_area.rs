pub struct Solution {}

impl Solution {
    pub fn compute_area(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> i32 {
        fn compute_overlapped_area(
            a: i32,
            b: i32,
            c: i32,
            d: i32,
            e: i32,
            f: i32,
            g: i32,
            h: i32,
        ) -> i32 {
            if c <= e || d <= f || h <= b {
                0
            } else {
                (std::cmp::min(c, g) - std::cmp::max(a, e))
                    * (std::cmp::min(d, h) - std::cmp::max(b, f))
            }
        }

        let overlapped = if e < a {
            compute_overlapped_area(e, f, g, h, a, b, c, d)
        } else {
            compute_overlapped_area(a, b, c, d, e, f, g, h)
        };

        (c - a) * (d - b) - overlapped + (g - e) * (h - f)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0223() {
        assert_eq!(45, Solution::compute_area(-3, 0, 3, 4, 0, -1, 9, 2));
        assert_eq!(45, Solution::compute_area(0, -1, 9, 2, -3, 0, 3, 4));
        assert_eq!(81, Solution::compute_area(0, 0, 9, 9, 1, 1, 8, 8));
        assert_eq!(45, Solution::compute_area(-3, 0, 3, 4, 0, 2, 9, 5));
        assert_eq!(82, Solution::compute_area(0, 0, 9, 9, 1, 0, 10, 1));
    }
}
