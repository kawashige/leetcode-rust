pub struct Solution {}

impl Solution {
    pub fn sum_of_good_integers(n: i32, k: i32) -> i32 {
        ((n - k).max(1)..=n + k).filter(|x| n & x == 0).sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3954() {
        assert_eq!(Solution::sum_of_good_integers(2, 3), 10);
        assert_eq!(Solution::sum_of_good_integers(5, 1), 0);
    }
}

fn main() {}
