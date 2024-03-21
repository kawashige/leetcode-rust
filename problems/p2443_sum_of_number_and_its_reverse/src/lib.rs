pub struct Solution {}

impl Solution {
    pub fn reverse(num: i32) -> i32 {
        let mut num = num;
        let mut reversed = 0;
        while 0 < num {
            reversed = reversed * 10 + num % 10;
            num /= 10;
        }
        reversed
    }

    pub fn sum_of_number_and_reverse(num: i32) -> bool {
        for i in 0..=num {
            if i + Self::reverse(i) == num {
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
    fn test_2443() {
        assert!(Solution::sum_of_number_and_reverse(0));
        assert!(Solution::sum_of_number_and_reverse(443));
        assert!(!Solution::sum_of_number_and_reverse(63));
        assert!(Solution::sum_of_number_and_reverse(181));
    }
}
