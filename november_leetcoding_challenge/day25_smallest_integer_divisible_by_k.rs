pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        if k % 2 == 0 {
            return -1;
        }
        let mut result = 1;
        let mut n = 1;
        let mut opened = HashSet::new();
        opened.insert(1);
        loop {
            let remainder = n % k;
            if remainder == 0 {
                break;
            }
            n = remainder * 10 + 1;
            if opened.contains(&n) {
                return -1;
            }
            opened.insert(n);
            result += 1;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day25() {
        assert_eq!(-1, Solution::smallest_repunit_div_by_k(15));
        assert_eq!(-1, Solution::smallest_repunit_div_by_k(6));
        assert_eq!(1, Solution::smallest_repunit_div_by_k(1));
        assert_eq!(-1, Solution::smallest_repunit_div_by_k(2));
        assert_eq!(3, Solution::smallest_repunit_div_by_k(3));
        assert_eq!(5, Solution::smallest_repunit_div_by_k(11111));
    }
}
