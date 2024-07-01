pub struct Solution {}

impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let mut p = 1;
        let mut next = 1;
        for _ in 0..time {
            if p == n {
                next = -1;
            } else if p == 1 {
                next = 1;
            }
            p += next;
        }
        p
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2582() {
        assert_eq!(Solution::pass_the_pillow(18, 38), 5);
        assert_eq!(Solution::pass_the_pillow(4, 5), 2);
        assert_eq!(Solution::pass_the_pillow(3, 2), 3);
    }
}
