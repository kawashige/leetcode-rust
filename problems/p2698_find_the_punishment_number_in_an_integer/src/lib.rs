pub struct Solution {}

impl Solution {
    pub fn check(s: &str, i: usize, sum: i32, num: i32, target: i32) -> bool {
        if i == s.len() {
            return sum + num == target;
        }
        let x = (s.as_bytes()[i] - b'0') as i32;
        Self::check(s, i + 1, sum + num, x, target)
            || Self::check(s, i + 1, sum, num * 10 + x, target)
    }

    pub fn punishment_number(n: i32) -> i32 {
        let mut result = 0;
        for i in 1..=n {
            let square = i * i;
            if Self::check(&square.to_string(), 0, 0, 0, i) {
                result += square;
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2698() {
        assert_eq!(Solution::punishment_number(1000), 182);
        assert_eq!(Solution::punishment_number(10), 182);
        assert_eq!(Solution::punishment_number(37), 1478);
    }
}
