pub struct Solution {}

impl Solution {
    pub fn check_good_integer(n: i32) -> bool {
        let mut sum = 0;
        let mut square_sum = 0;

        let mut n = n;
        while 0 < n {
            let d = n % 10;
            n /= 10;
            sum += d;
            square_sum += d * d;
        }

        50 <= square_sum - sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3959() {
        assert!(!Solution::check_good_integer(1000));
        assert!(Solution::check_good_integer(18));
    }
}

fn main() {}
