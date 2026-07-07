pub struct Solution {}

impl Solution {
    pub fn is_prime(n: i32) -> bool {
        if n == 1 {
            return false;
        }
        for i in 2..=((n as f64).sqrt() as i32) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
    pub fn complete_prime(num: i32) -> bool {
        let num_s = num.to_string();
        for i in 0..num_s.len() {
            if !Self::is_prime(num_s[..=i].parse::<i32>().unwrap()) {
                return false;
            }
            if !Self::is_prime(num_s[i..].parse::<i32>().unwrap()) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3765() {
        assert!(Solution::complete_prime(23));
        assert!(!Solution::complete_prime(39));
        assert!(Solution::complete_prime(7));
    }
}

fn main() {}
