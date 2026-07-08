use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> Vec<i32> {
        let max = *nums.iter().max().unwrap();
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(3);
        for i in 2..=max {
            let chars = format!("{:b}", i).chars().collect::<Vec<_>>();
            let p1 = i32::from_str_radix(
                &chars.iter().chain(chars.iter().rev()).collect::<String>(),
                2,
            )
            .unwrap();
            set.insert(p1);

            let p2 = i32::from_str_radix(
                &chars
                    .iter()
                    .chain(chars[..chars.len() - 1].iter().rev())
                    .collect::<String>(),
                2,
            )
            .unwrap();
            set.insert(p2);
        }

        let mut palindrome = set.into_iter().collect::<Vec<_>>();
        palindrome.sort_unstable();

        nums.into_iter()
            .map(|n| match palindrome.binary_search(&n) {
                Ok(_) => 0,
                Err(i) if i == 0 => palindrome[i] - n,
                Err(i) => (palindrome[i] - n).min(n - palindrome[i - 1]),
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3766() {
        assert_eq!(Solution::min_operations(vec![1624]), vec![5]);
        assert_eq!(Solution::min_operations(vec![1, 2, 4]), vec![0, 1, 1]);
        assert_eq!(Solution::min_operations(vec![6, 7, 12]), vec![1, 0, 3]);
    }
}

fn main() {}
