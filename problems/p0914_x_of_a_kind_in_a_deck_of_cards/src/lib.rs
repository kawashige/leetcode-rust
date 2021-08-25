pub struct Solution {}

impl Solution {
    pub fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }

    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        deck.into_iter()
            .fold(vec![0; 10_001], |mut count, x| {
                count[x as usize] += 1;
                count
            })
            .into_iter()
            .fold(None, |acc, x| {
                if x == 0 {
                    acc
                } else if acc.is_none() {
                    Some(x)
                } else {
                    Some(Self::gcd(acc.unwrap(), x))
                }
            })
            .unwrap()
            > 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0914() {
        assert!(Solution::has_groups_size_x(vec![1, 2, 3, 4, 4, 3, 2, 1]));
        assert!(!Solution::has_groups_size_x(vec![1, 1, 1, 2, 2, 2, 3, 3]));
        assert!(!Solution::has_groups_size_x(vec![1]));
        assert!(Solution::has_groups_size_x(vec![1, 1]));
        assert!(Solution::has_groups_size_x(vec![1, 1, 2, 2, 2, 2]));
    }
}
