pub struct Solution {}

impl Solution {
    pub fn can_alice_win(n: i32) -> bool {
        let mut n = n;
        let mut r = 10;

        for i in 0.. {
            if r == 0 || n < r {
                return i % 2 == 1;
            }
            n -= r;
            r -= 1;
        }

        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3360() {
        assert!(Solution::can_alice_win(12));
        assert!(!Solution::can_alice_win(1));
    }
}
