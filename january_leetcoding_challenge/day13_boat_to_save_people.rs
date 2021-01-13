pub struct Solution {}

impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort_unstable();

        let mut result = 0;
        let mut i = 0;
        let mut j = people.len() - 1;
        while i <= j {
            result += 1;
            if i == j || j == 0 {
                break;
            }
            if people[i] <= limit - people[j] {
                i += 1;
            }
            j -= 1;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day13() {
        assert_eq!(1, Solution::num_rescue_boats(vec![1], 3));
        assert_eq!(1, Solution::num_rescue_boats(vec![1, 2], 3));
        assert_eq!(3, Solution::num_rescue_boats(vec![3, 2, 2, 1], 3));
        assert_eq!(4, Solution::num_rescue_boats(vec![3, 5, 3, 4], 5));
    }
}
