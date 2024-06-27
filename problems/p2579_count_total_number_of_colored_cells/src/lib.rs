pub struct Solution {}

impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        let mut count = 1;
        for i in 2..=n {
            count += i as i64 * 4 - 4
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2579() {
        assert_eq!(Solution::colored_cells(100000), 1);
        assert_eq!(Solution::colored_cells(1), 1);
        assert_eq!(Solution::colored_cells(2), 5);
    }
}
