pub struct Solution {}

impl Solution {
    pub fn get_least_frequent_digit(n: i32) -> i32 {
        let mut count = [0; 10];
        let mut n = n as usize;
        while 0 < n {
            count[n % 10] += 1;
            n /= 10
        }
        count
            .iter()
            .zip(0..)
            .filter(|(c, _)| &&0 < c)
            .min()
            .unwrap()
            .1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3663() {
        assert_eq!(Solution::get_least_frequent_digit(1553322), 1);
        assert_eq!(Solution::get_least_frequent_digit(723344511), 2);
    }
}
