pub struct Solution {}

impl Solution {
    pub fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }

    pub fn lcm(a: i32, b: i32) -> i32 {
        a / Self::gcd(a, b) * b
    }

    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();

        for num in nums {
            let mut num = num;
            while !result.is_empty() && 1 < Self::gcd(*result.last().unwrap(), num) {
                num = Self::lcm(result.pop().unwrap(), num);
            }
            result.push(num);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2197() {
        assert_eq!(
            Solution::replace_non_coprimes(vec![6, 4, 3, 2, 7, 6, 2]),
            vec![12, 7, 6]
        );
        assert_eq!(
            Solution::replace_non_coprimes(vec![2, 2, 1, 1, 3, 3, 3]),
            vec![2, 1, 1, 3]
        );
    }
}
