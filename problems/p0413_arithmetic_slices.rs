pub struct Solution {}

impl Solution {
    pub fn number_of_arithmetic_slices(a: Vec<i32>) -> i32 {
        if a.len() < 3 {
            return 0;
        }

        let mut diff = a[1] - a[0];
        let mut count = 1;
        let mut result = 0;
        for i in 2..a.len() {
            if a[i] - a[i - 1] == diff {
                count += 1;
                if 2 <= count {
                    result += count - 1;
                }
            } else {
                diff = a[i] - a[i - 1];
                count = 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0413() {
        assert_eq!(3, Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4]));
        assert_eq!(1, Solution::number_of_arithmetic_slices(vec![1, 2, 3]));
        assert_eq!(0, Solution::number_of_arithmetic_slices(vec![1]));
        assert_eq!(
            4,
            Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4, 2, 3, 8, 13])
        );
    }
}
