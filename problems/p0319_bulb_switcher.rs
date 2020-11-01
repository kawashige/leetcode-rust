pub struct Solution {}

impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        (1..=n).take_while(|i| i * i <= n).count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0319() {
        assert_eq!(3162, Solution::bulb_switch(10000000));
        assert_eq!(1, Solution::bulb_switch(1));
        assert_eq!(0, Solution::bulb_switch(0));
        assert_eq!(1, Solution::bulb_switch(3));
    }
}
