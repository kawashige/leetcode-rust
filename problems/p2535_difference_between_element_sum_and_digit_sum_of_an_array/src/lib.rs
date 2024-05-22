pub struct Solution {}

impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        let (sum, digit_sum) = nums.into_iter().fold((0, 0), |(sum, digit_sum), num| {
            let mut remains = num;
            let mut new_digit_sum = digit_sum;
            while 0 < remains {
                new_digit_sum += remains % 10;
                remains /= 10;
            }
            (sum + num, new_digit_sum)
        });
        (sum - digit_sum).abs()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2535() {
        assert_eq!(Solution::difference_of_sum(vec![1, 15, 6, 3]), 9);
        assert_eq!(Solution::difference_of_sum(vec![1, 2, 3, 4]), 0);
    }
}
