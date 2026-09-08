pub struct Solution {}

impl Solution {
    pub fn count_commas(n: i32) -> i32 {
        (n - 999).max(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3870() {
        assert_eq!(Solution::count_commas(1002), 3);
        assert_eq!(Solution::count_commas(998), 0);
    }
}

fn main() {}
