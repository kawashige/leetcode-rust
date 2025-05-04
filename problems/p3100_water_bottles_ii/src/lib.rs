pub struct Solution {}

impl Solution {
    pub fn max_bottles_drunk(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut result = num_bottles;
        let mut empty = num_bottles;
        let mut num_exchange = num_exchange;

        while num_exchange <= empty {
            result += 1;
            empty -= num_exchange;
            empty += 1;
            num_exchange += 1;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3100() {
        assert_eq!(Solution::max_bottles_drunk(13, 6), 15);
        assert_eq!(Solution::max_bottles_drunk(10, 3), 13);
    }
}
