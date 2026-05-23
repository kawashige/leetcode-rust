pub struct Solution {}

impl Solution {
    pub fn max_sum_of_squares(num: i32, sum: i32) -> String {
        let mut result = String::new();
        let mut sum = sum;

        for _ in 0..num {
            let d = sum.min(9);
            result.push((d as u8 + b'0') as char);
            sum -= d;
        }

        if 0 < sum { String::new() } else { result }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3727() {
        assert_eq!(Solution::max_sum_of_squares(2, 3), "30".to_string());
        assert_eq!(Solution::max_sum_of_squares(2, 17), "98".to_string());
        assert_eq!(Solution::max_sum_of_squares(1, 10), "".to_string());
    }
}

fn main() {}
