pub struct Solution {}

impl Solution {
    pub fn mirror_distance(n: i32) -> i32 {
        let mut x = n;
        let mut reversed_n = 0;
        while 0 < x {
            reversed_n = reversed_n * 10 + x % 10;
            x /= 10
        }
        (n - reversed_n).abs()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3783() {
        assert_eq!(Solution::mirror_distance(25), 27);
        assert_eq!(Solution::mirror_distance(10), 9);
        assert_eq!(Solution::mirror_distance(7), 0);
    }
}
