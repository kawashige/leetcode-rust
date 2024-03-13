pub struct Solution {}

impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let total = n * (n + 1) / 2;
        let mut sum = 0;

        for i in 1..=n {
            println!("i: {}, sum: {}, total - sum: {}", i, sum, total - sum);
            if sum + i == total - sum {
                return i;
            }
            sum += i;
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2485() {
        assert_eq!(Solution::pivot_integer(8), 6);
        assert_eq!(Solution::pivot_integer(1), 1);
        assert_eq!(Solution::pivot_integer(4), -1);
    }
}
