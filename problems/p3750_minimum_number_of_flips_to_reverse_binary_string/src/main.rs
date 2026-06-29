pub struct Solution {}

impl Solution {
    pub fn minimum_flips(n: i32) -> i32 {
        let s = format!("{:b}", n).trim_start_matches('0').to_string();
        (0..s.len() / 2)
            .filter(|i| s.as_bytes()[*i] != s.as_bytes()[s.len() - 1 - i])
            .count() as i32
            * 2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3750() {
        assert_eq!(Solution::minimum_flips(7), 0);
        assert_eq!(Solution::minimum_flips(10), 4);
    }
}

fn main() {}
