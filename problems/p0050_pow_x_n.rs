pub struct Solution {}

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        use std::collections::HashMap;

        if n == 0 {
            return 1.0;
        }

        let mut result = 1.0;
        let mut multiplier: HashMap<i64, f64> = vec![(1, x)].into_iter().collect();
        let mut current = 0;
        let mut num = (n as i64).abs();
        let mut mul = x;
        while 0 < num {
            // println!("result: {}, num: {}, current: {}", result, num, current);
            while num < current {
                current /= 2;
                mul = multiplier[&current];
            }
            result *= mul;
            num -= if current == 0 { 1 } else { current };
            mul = result;
            current = if current == 0 { 1 } else { current * 2 };
            multiplier.insert(current, result);
        }

        if n < 0 {
            ((1.0 / result) * 100000.0).round() / 100000.0
        } else {
            (result * 100000.0).round() / 100000.0
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0050() {
        assert_eq!(1024.00000, Solution::my_pow(2.00000, 10));
        assert_eq!(9.26100, Solution::my_pow(2.10000, 3));
        assert_eq!(0.25000, Solution::my_pow(2.00000, -2));
        assert_eq!(1.0, Solution::my_pow(2.00000, 0));
        assert_eq!(700.28148, Solution::my_pow(8.88023, 3));
        assert_eq!(54.83508, Solution::my_pow(0.44894, -5));
        assert_eq!(0.00000, Solution::my_pow(0.00001, 2147483647));
        assert_eq!(0.00000, Solution::my_pow(2.00000, -2147483648));
    }
}
