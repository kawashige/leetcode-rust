pub struct Solution {}

impl Solution {
    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        let mut mul = a % 1337;

        if mul < 1 {
            return mul;
        }

        let mut result = 1;
        for j in (0..b.len()).rev() {
            let mut n = 1;
            for i in 1..10 {
                n = (mul * n) % 1337;
                if i == b[j] {
                    result = result * n % 1337
                }
            }
            mul = (n * mul) % 1337;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0372() {
        assert_eq!(8, Solution::super_pow(2, vec![3]));
        assert_eq!(1024, Solution::super_pow(2, vec![1, 0]));
        assert_eq!(1, Solution::super_pow(1, vec![4, 3, 3, 8, 5, 2]));
        assert_eq!(1198, Solution::super_pow(2147483647, vec![2, 0, 0]));
    }
}
