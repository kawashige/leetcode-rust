pub struct Solution {}

impl Solution {
    pub fn min_changes(n: i32, k: i32) -> i32 {
        let mut count = 0;
        for i in 0..32 {
            if n & 1 << i != 0 && k & 1 << i == 0 {
                count += 1;
            } else if n & 1 << i == 0 && k & 1 << i != 0 {
                return -1;
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3226() {
        assert_eq!(Solution::min_changes(13, 4), 2);
        assert_eq!(Solution::min_changes(21, 21), 0);
        assert_eq!(Solution::min_changes(14, 13), -1);
    }
}
