pub struct Solution {}

impl Solution {
    pub fn mirror_reflection(p: i32, q: i32) -> i32 {
        let mut current = (p, q);
        let mut direction = 1;

        loop {
            println!(
                "p: {}, q: {}, direction: {}, current: {:?}",
                p, q, direction, current
            );
            if current == (p, 0) {
                return 0;
            } else if current == (p, p) {
                return 1;
            } else if current == (0, p) {
                return 2;
            }

            current.0 = if current.0 == 0 { p } else { 0 };
            current.1 += direction * q;
            if p < current.1 {
                current.1 = p - (current.1 - p);
                direction *= -1;
            } else if current.1 < 0 {
                current.1 *= -1;
                direction *= -1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day17() {
        assert_eq!(1, Solution::mirror_reflection(2, 2));
        assert_eq!(2, Solution::mirror_reflection(2, 1));
        assert_eq!(0, Solution::mirror_reflection(3, 2));
        assert_eq!(2, Solution::mirror_reflection(4, 2));
    }
}
