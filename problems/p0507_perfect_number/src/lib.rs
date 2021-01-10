pub struct Solution {}

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num < 2 {
            return false;
        }
        let n = (num as f32).sqrt() as i32;
        1 + (2..=n)
            .filter(|i| num % i == 0)
            .map(|i| i + num / i)
            .sum::<i32>()
            == num
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0507() {
        assert!(Solution::check_perfect_number(28));
        assert!(Solution::check_perfect_number(6));
        assert!(Solution::check_perfect_number(496));
        assert!(Solution::check_perfect_number(8128));
        assert!(!Solution::check_perfect_number(2));
        assert!(!Solution::check_perfect_number(0));
        assert!(!Solution::check_perfect_number(1));
    }
}
