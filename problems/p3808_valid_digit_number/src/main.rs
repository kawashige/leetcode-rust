pub struct Solution {}

impl Solution {
    pub fn valid_digit(n: i32, x: i32) -> bool {
        let s = n.to_string().chars().collect::<Vec<_>>();
        let c = (x as u8 + b'0') as char;
        s[0] != c && s.contains(&c)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3808() {
        assert!(Solution::valid_digit(101, 0));
        assert!(!Solution::valid_digit(232, 2));
        assert!(!Solution::valid_digit(5, 1));
    }
}

fn main() {}
