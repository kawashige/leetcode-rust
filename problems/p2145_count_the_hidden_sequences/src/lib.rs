pub struct Solution {}

impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut low: i64 = 0;
        let mut high = 0;
        let mut v = 0;

        for d in differences {
            v += d as i64;
            low = low.min(v);
            high = high.max(v);
        }

        (upper as i64 - (high + lower as i64 - low) + 1).max(0) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2145() {
        assert_eq!(Solution::number_of_arrays(vec![1, -3, 4], 1, 6), 2);
        assert_eq!(Solution::number_of_arrays(vec![3, -4, 5, 1, -2], -4, 5), 4);
        assert_eq!(Solution::number_of_arrays(vec![4, -7, 2], 3, 6), 0);
    }
}
