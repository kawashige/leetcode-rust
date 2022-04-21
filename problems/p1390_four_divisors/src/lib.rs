pub struct Solution {}

impl Solution {
    pub fn four_divisors(num: i32) -> i32 {
        if num == 1 {
            return 0;
        }

        let mut sum = 1 + num;
        let mut count = 2;

        for i in 2..=((num as f64).sqrt() as i32) {
            if num % i == 0 {
                if num / i == i {
                    sum += i;
                    count += 1;
                } else {
                    sum += i + num / i;
                    count += 2;
                }
            }
        }
        if count == 4 {
            sum
        } else {
            0
        }
    }

    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        nums.into_iter().map(|num| Self::four_divisors(num)).sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1390() {
        assert_eq!(Solution::sum_four_divisors(vec![21, 4, 7]), 32);
        assert_eq!(Solution::sum_four_divisors(vec![21, 21]), 64);
        assert_eq!(Solution::sum_four_divisors(vec![1, 2, 3, 4, 5]), 0);
    }
}
