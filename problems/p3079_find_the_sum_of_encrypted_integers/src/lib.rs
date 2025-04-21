pub struct Solution {}

impl Solution {
    pub fn sum_of_encrypted_int(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .map(|n| {
                let mut n = n;
                let mut max_digit = 0;
                let mut len = 0;
                while 0 < n {
                    max_digit = max_digit.max(n % 10);
                    len += 1;
                    n /= 10;
                }
                let mut x = 0;
                for _ in 0..len {
                    x = x * 10 + max_digit;
                }
                x
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3079() {
        assert_eq!(Solution::sum_of_encrypted_int(vec![1, 2, 3]), 6);
        assert_eq!(Solution::sum_of_encrypted_int(vec![10, 21, 31]), 66);
    }
}
