pub struct Solution {}

impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let mut counts = vec![0];
        while counts.len() * 2 <= num as usize + 1 {
            counts.append(&mut counts.iter().map(|n| *n + 1).collect())
        }
        let rest = num as usize + 1 - counts.len();
        if 0 < rest {
            counts.append(&mut counts.iter().take(rest).map(|n| *n + 1).collect())
        }

        counts
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0338() {
        assert_eq!(vec![0, 1, 1], Solution::count_bits(2));
        assert_eq!(vec![0, 1, 1, 2, 1, 2], Solution::count_bits(5));
    }
}
