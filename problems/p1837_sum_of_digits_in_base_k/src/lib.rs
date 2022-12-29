pub struct Solution {}

impl Solution {
    pub fn sum_base(n: i32, k: i32) -> i32 {
        let mut remains = n;
        let mut result = 0;
        while 0 < remains {
            result += remains % k;
            remains /= k;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1837() {
        assert_eq!(Solution::sum_base(34, 6), 9);
        assert_eq!(Solution::sum_base(10, 10), 1);
    }
}
