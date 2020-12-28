pub struct Solution {}

impl Solution {
    pub fn reach_number(mut target: i32) -> i32 {
        if target == 0 {
            return 0;
        }
        target = target.abs();
        let mut n = 0;
        let mut i = 0;
        while n < target {
            i += 1;
            n += i;
        }
        let j = if i % 2 == 0 { i } else { i + 1 } / 2;
        if j % 2 == target % 2 {
            i
        } else {
            j * 2 + 1
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day28() {
        assert_eq!(3, Solution::reach_number(4));
        assert_eq!(2, Solution::reach_number(3));
        assert_eq!(3, Solution::reach_number(2));
        assert_eq!(0, Solution::reach_number(0));
        assert_eq!(5, Solution::reach_number(13));
        assert_eq!(7, Solution::reach_number(14));
        assert_eq!(1, Solution::reach_number(1));
    }
}
