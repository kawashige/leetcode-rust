pub struct Solution {}

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        let mut count = 0;

        for i in 0..colors.len() + k as usize - 3 {
            if colors[(i + colors.len() - 1) % colors.len()] != colors[i % colors.len()]
                && colors[i % colors.len()] != colors[(i + 1) % colors.len()]
            {
                count += 1;
            } else {
                count = 0;
            }
            if k - 2 <= count {
                result += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3208() {
        assert_eq!(
            Solution::number_of_alternating_groups(vec![0, 1, 0, 1, 0], 3),
            3
        );
        assert_eq!(
            Solution::number_of_alternating_groups(vec![0, 1, 0, 0, 1, 0, 1], 6),
            2
        );
        assert_eq!(
            Solution::number_of_alternating_groups(vec![1, 1, 0, 1], 4),
            0
        );
    }
}
