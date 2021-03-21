pub struct Solution {}

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        fn count(n: i32) -> [usize; 10] {
            n.to_string().chars().fold([0; 10], |mut count, c| {
                count[c as usize - 0x30] += 1;
                count
            })
        }

        let a = count(n);
        for i in 0..31 {
            if a == count(1 << i) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day21() {
        assert!(Solution::reordered_power_of2(1));
        assert!(!Solution::reordered_power_of2(10));
        assert!(Solution::reordered_power_of2(16));
        assert!(!Solution::reordered_power_of2(24));
        assert!(Solution::reordered_power_of2(46));
    }
}
