pub struct Solution {}

impl Solution {
    pub fn largest_integer(num: i32) -> i32 {
        let mut parity = vec![];
        let mut odds = vec![];
        let mut evens = vec![];

        let mut num = num;
        while 0 < num {
            if num % 2 == 0 {
                evens.push(num % 10);
                parity.push(0);
            } else {
                odds.push(num % 10);
                parity.push(1);
            }
            num /= 10;
        }

        evens.sort_unstable();
        odds.sort_unstable();

        let mut result = 0;
        for i in (0..parity.len()).rev() {
            result = result * 10
                + if parity[i] == 0 {
                    evens.pop().unwrap()
                } else {
                    odds.pop().unwrap()
                };
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2231() {
        assert_eq!(Solution::largest_integer(1234), 3412);
        assert_eq!(Solution::largest_integer(65875), 87655);
    }
}
