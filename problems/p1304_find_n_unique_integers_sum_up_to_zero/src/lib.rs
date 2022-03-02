pub struct Solution {}

impl Solution {
    pub fn sum_zero(mut n: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(n as usize);
        if n % 2 == 1 {
            result.push(0);
            n -= 1;
        }

        for i in 0..(n / 2) {
            result.push(i + 1);
            result.push(-(i + 1));
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1304() {
        assert_eq!(Solution::sum_zero(5), [0, 1, -1, 2, -2]);
        assert_eq!(Solution::sum_zero(3), [0, 1, -1]);
        assert_eq!(Solution::sum_zero(2), [1, -1]);
        assert_eq!(Solution::sum_zero(1), [0]);
    }
}
