pub struct Solution {}

impl Solution {
    pub fn minimum_levels(possible: Vec<i32>) -> i32 {
        let total = possible
            .iter()
            .map(|p| if p == &1 { 1 } else { -1 })
            .sum::<i32>();
        let mut p = 0;
        for i in 0..possible.len() - 1 {
            p += if possible[i] == 1 { 1 } else { -1 };
            if total - p < p {
                return i as i32 + 1;
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3096() {
        assert_eq!(Solution::minimum_levels(vec![1, 0, 1, 0]), 1);
        assert_eq!(Solution::minimum_levels(vec![1, 1, 1, 1, 1]), 3);
        assert_eq!(Solution::minimum_levels(vec![0, 0]), -1);
    }
}
