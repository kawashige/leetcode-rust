pub struct Solution {}

impl Solution {
    pub fn max_digit_range(nums: Vec<i32>) -> i32 {
        let mut largest_digit_range = 0;
        let mut result = 0;
        for n in &nums {
            let mut remains = *n;
            let (mut max, mut min) = (0, 9);
            while 0 < remains {
                let d = remains % 10;
                remains /= 10;
                max = max.max(d);
                min = min.min(d);
            }
            let digit_range = max - min;
            if largest_digit_range < digit_range {
                result = *n;
                largest_digit_range = digit_range
            } else if largest_digit_range == digit_range {
                result += n;
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3982() {
        assert_eq!(Solution::max_digit_range(vec![5724, 111, 350]), 6074);
        assert_eq!(Solution::max_digit_range(vec![90, 900]), 990);
    }
}

fn main() {}
