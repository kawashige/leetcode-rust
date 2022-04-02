pub struct Solution {}

impl Solution {
    pub fn find(num: i32) -> Vec<i32> {
        for x in (1..=(num as f64).sqrt() as i32).rev() {
            if num % x == 0 {
                return vec![x, num / x];
            }
        }
        unreachable!()
    }

    pub fn closest_divisors(num: i32) -> Vec<i32> {
        let d1 = Self::find(num + 1);
        let d2 = Self::find(num + 2);
        if d1[1] - d1[0] < d2[1] - d2[0] {
            d1
        } else {
            d2
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1362() {
        assert_eq!(Solution::closest_divisors(8), vec![3, 3]);
        assert_eq!(Solution::closest_divisors(123), vec![5, 25]);
        assert_eq!(Solution::closest_divisors(999), vec![25, 40]);
    }
}
