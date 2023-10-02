pub struct Solution {}

impl Solution {
    pub fn count_even(num: i32) -> i32 {
        let mut count = 0;

        for i in 1..=num {
            let mut num = i;
            let mut sum = 0;
            while 0 < num {
                sum += num % 10;
                num /= 10;
            }
            if sum % 2 == 0 {
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2180() {
        assert_eq!(Solution::count_even(4), 2);
        assert_eq!(Solution::count_even(30), 14);
    }
}
