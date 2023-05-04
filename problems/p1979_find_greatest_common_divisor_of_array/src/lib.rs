pub struct Solution {}

impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let mut min = std::i32::MAX;
        let mut max = std::i32::MIN;

        for num in nums {
            min = min.min(num);
            max = max.max(num);
        }

        for x in (1..=max).rev() {
            if min % x == 0 && max % x == 0 {
                return x;
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1979() {
        assert_eq!(Solution::find_gcd(vec![2, 5, 6, 9, 10]), 2);
        assert_eq!(Solution::find_gcd(vec![7, 5, 6, 8, 3]), 1);
        assert_eq!(Solution::find_gcd(vec![3, 3]), 3);
    }
}
