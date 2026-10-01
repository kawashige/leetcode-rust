pub struct Solution {}

impl Solution {
    pub fn first_unique_even(nums: Vec<i32>) -> i32 {
        let mut count = [0; 101];
        for n in &nums {
            count[*n as usize] += 1;
        }

        nums.into_iter()
            .find(|n| n % 2 == 0 && count[*n as usize] == 1)
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3866() {
        assert_eq!(Solution::first_unique_even(vec![3, 4, 2, 5, 4, 6]), 2);
        assert_eq!(Solution::first_unique_even(vec![4, 4]), -1);
    }
}

fn main() {}
