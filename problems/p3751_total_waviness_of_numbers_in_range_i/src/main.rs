pub struct Solution {}

impl Solution {
    pub fn count_wabiness(n: i32) -> i32 {
        if n < 100 {
            return 0;
        }

        let digits = n.to_string().chars().collect::<Vec<_>>();
        (1..digits.len() - 1)
            .filter(|i| {
                (digits[i - 1] < digits[*i] && digits[i + 1] < digits[*i])
                    || (digits[i - 1] > digits[*i] && digits[i + 1] > digits[*i])
            })
            .count() as i32
    }

    pub fn total_waviness(num1: i32, num2: i32) -> i32 {
        (num1..=num2).map(|x| Self::count_wabiness(x)).sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3751() {
        assert_eq!(Solution::total_waviness(120, 130), 3);
        assert_eq!(Solution::total_waviness(198, 202), 3);
        assert_eq!(Solution::total_waviness(4848, 4848), 2);
    }
}

fn main() {}
