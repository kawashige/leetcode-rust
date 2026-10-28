pub struct Solution {}

impl Solution {
    pub fn consecutive_set_bits(n: i32) -> bool {
        let mut found = false;
        for i in 1..32 {
            if n & 1 << i != 0 && n & 1 << (i - 1) != 0 {
                if found {
                    return false;
                }
                found = true;
            }
        }

        found
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3950() {
        assert!(Solution::consecutive_set_bits(6));
        assert!(!Solution::consecutive_set_bits(5));
    }
}

fn main() {}
