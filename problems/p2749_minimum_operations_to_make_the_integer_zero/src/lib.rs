pub struct Solution {}

impl Solution {
    pub fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
        let mut num1 = num1 as i64;
        let num2 = num2 as i64;

        for i in 0..60 {
            num1 -= num2;
            if num1 < 0 {
                break;
            }
            let mut count = 0_usize;
            for i in (0..64).rev() {
                count *= 2;
                if num1 & 1 << i != 0 {
                    count += 1;
                }
            }
            if (num1.count_ones() as usize..=count).contains(&(i as usize + 1)) {
                return i as i32 + 1;
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2749() {
        assert_eq!(Solution::make_the_integer_zero(3, -2), 3);
        assert_eq!(Solution::make_the_integer_zero(5, 7), -1);
    }
}
