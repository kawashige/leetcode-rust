pub struct Solution {}

impl Solution {
    pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
        let mut n = 0;
        let mut remains = x;
        while 0 < remains {
            n += remains % 10;
            remains /= 10;
        }
        if x % n == 0 {
            n
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3099() {
        assert_eq!(Solution::sum_of_the_digits_of_harshad_number(18), 9);
        assert_eq!(Solution::sum_of_the_digits_of_harshad_number(23), -1);
    }
}
