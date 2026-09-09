pub struct Solution {}

impl Solution {
    pub fn count_commas(n: i64) -> i64 {
        let mut x = 1000;
        let mut count = 0;

        while x <= n {
            count += n - (x - 1);
            x *= 1000;
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3871() {
        assert_eq!(Solution::count_commas(1002), 3);
        assert_eq!(Solution::count_commas(998), 0);
    }
}

fn main() {}
