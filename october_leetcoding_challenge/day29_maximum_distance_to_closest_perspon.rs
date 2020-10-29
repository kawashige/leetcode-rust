pub struct Solution {}

impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let mut result = 1;
        let mut i = 0;
        while i < seats.len() {
            if seats[i] == 1 {
                i += 1;
                continue;
            }

            let mut j = i;
            while j + 1 < seats.len() && seats[j + 1] == 0 {
                j += 1;
            }

            let mut d = j - i + 1;
            if i != 0 && j != seats.len() - 1 {
                d = d - d / 2;
            }
            result = std::cmp::max(result, d);
            i = j + 1;
        }
        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day29() {
        assert_eq!(3, Solution::max_dist_to_closest(vec![1, 0, 1, 0, 0, 0]));
        assert_eq!(3, Solution::max_dist_to_closest(vec![0, 0, 0, 1, 0, 1]));
        assert_eq!(2, Solution::max_dist_to_closest(vec![1, 0, 0, 0, 1, 0, 1]));
        assert_eq!(2, Solution::max_dist_to_closest(vec![1, 0, 0, 0, 0, 1]));
        assert_eq!(3, Solution::max_dist_to_closest(vec![1, 0, 0, 0, 0, 0, 1]));
        assert_eq!(3, Solution::max_dist_to_closest(vec![1, 0, 0, 0]));
        assert_eq!(1, Solution::max_dist_to_closest(vec![0, 1]));
        assert_eq!(2, Solution::max_dist_to_closest(vec![0, 1, 0, 0]));
    }
}
