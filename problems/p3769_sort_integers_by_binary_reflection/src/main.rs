pub struct Solution {}

impl Solution {
    pub fn sort_by_reflection(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable_by_key(|n| {
            let reversed = i32::from_str_radix(
                &format!("{:b}", n)
                    .trim_start_matches('0')
                    .chars()
                    .rev()
                    .collect::<String>(),
                2,
            )
            .unwrap();
            (reversed, *n)
        });
        nums
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3769() {
        assert_eq!(Solution::sort_by_reflection(vec![4, 5, 4]), vec![4, 4, 5]);
        assert_eq!(
            Solution::sort_by_reflection(vec![3, 6, 5, 8]),
            vec![8, 3, 6, 5]
        );
    }
}

fn main() {}
