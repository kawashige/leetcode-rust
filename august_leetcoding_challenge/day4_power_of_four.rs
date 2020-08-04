pub struct Solution {}

impl Solution {
    pub fn is_power_of_four(num: i32) -> bool {
        let mut r = num;
        while r != 0 && r % 4 == 0 {
            r /= 4;
        }
        match r {
            1 => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day4() {
        assert_eq!(true, Solution::is_power_of_four(4));
        assert_eq!(true, Solution::is_power_of_four(16));
        assert_eq!(false, Solution::is_power_of_four(6));
        assert_eq!(false, Solution::is_power_of_four(0));
    }
}
