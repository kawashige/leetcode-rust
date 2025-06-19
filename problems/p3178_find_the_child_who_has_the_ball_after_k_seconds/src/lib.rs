pub struct Solution {}

impl Solution {
    pub fn number_of_child(n: i32, k: i32) -> i32 {
        let mut c = 0;
        let mut d = 1;
        for _ in 0..k {
            if c + d == n || c + d == -1 {
                d *= -1;
            }
            c += d;
        }
        c
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3178() {
        assert_eq!(Solution::number_of_child(3, 5), 1);
        assert_eq!(Solution::number_of_child(5, 6), 2);
        assert_eq!(Solution::number_of_child(4, 2), 2);
    }
}
