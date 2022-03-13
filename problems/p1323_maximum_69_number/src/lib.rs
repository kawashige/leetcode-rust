pub struct Solution {}

impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        let mut changed = false;

        num.to_string()
            .chars()
            .fold(0, |num, c| match (c, changed) {
                ('6', false) => {
                    changed = true;
                    num * 10 + 9
                }
                ('6', _) => num * 10 + 6,
                ('9', _) => num * 10 + 9,
                _ => unreachable!(),
            })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1323() {
        assert_eq!(Solution::maximum69_number(9669), 9969);
        assert_eq!(Solution::maximum69_number(9996), 9999);
        assert_eq!(Solution::maximum69_number(9999), 9999);
    }
}
