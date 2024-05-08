pub struct Solution {}

impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        let mut remains = num;
        let mut count = 0;
        while 0 < remains {
            if num % (remains % 10) == 0 {
                count += 1;
            }
            remains /= 10;
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2520() {
        assert_eq!(Solution::count_digits(7), 1);
        assert_eq!(Solution::count_digits(121), 2);
        assert_eq!(Solution::count_digits(1248), 4);
    }
}
