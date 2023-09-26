pub struct Solution {}

impl Solution {
    pub fn count_operations(num1: i32, num2: i32) -> i32 {
        let mut num1 = num1;
        let mut num2 = num2;
        let mut count = 0;

        while 0 < num1 && 0 < num2 {
            if num2 <= num1 {
                num1 -= num2;
            } else {
                num2 -= num1;
            }
            count += 1;
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2169() {
        assert_eq!(Solution::count_operations(2, 3), 3);
        assert_eq!(Solution::count_operations(10, 10), 1);
    }
}
