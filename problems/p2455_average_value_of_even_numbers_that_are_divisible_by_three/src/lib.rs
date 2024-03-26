pub struct Solution {}

impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let (sum, count) = nums.into_iter().fold((0, 0), |(sum, count), num| {
            if num % 6 == 0 {
                (sum + num, count + 1)
            } else {
                (sum, count)
            }
        });
        if count == 0 {
            0
        } else {
            sum / count
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2455() {
        assert_eq!(Solution::average_value(vec![1, 3, 6, 10, 12, 15]), 9);
        assert_eq!(Solution::average_value(vec![1, 2, 4, 7, 10]), 0);
    }
}
