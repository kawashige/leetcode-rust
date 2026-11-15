pub struct Solution {}

impl Solution {
    pub fn count_digit_occurrences(nums: Vec<i32>, digit: i32) -> i32 {
        let digit_c = (b'0' + digit as u8) as char;
        nums.into_iter()
            .map(|d| d.to_string().chars().filter(|c| c == &digit_c).count())
            .sum::<usize>() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3895() {
        assert_eq!(
            Solution::count_digit_occurrences(vec![12, 54, 32, 22], 2),
            4
        );
        assert_eq!(Solution::count_digit_occurrences(vec![1, 34, 7], 9), 0);
    }
}

fn main() {}
