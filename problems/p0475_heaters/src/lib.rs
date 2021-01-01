pub struct Solution {}

impl Solution {
    pub fn find_radius(mut houses: Vec<i32>, mut heaters: Vec<i32>) -> i32 {
        houses.sort();
        heaters.sort();

        let mut radius = 0;
        let mut i = 0;
        for h in houses {
            while i != heaters.len() - 1 && (heaters[i] - h).abs() >= (heaters[i + 1] - h).abs() {
                i += 1;
            }
            radius = std::cmp::max(radius, (heaters[i] - h).abs())
        }
        radius
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0475() {
        assert_eq!(
            498,
            Solution::find_radius(
                vec![1, 1, 1, 1, 1, 1, 999, 999, 999, 999, 999],
                vec![499, 500, 501]
            )
        );
        assert_eq!(1, Solution::find_radius(vec![1, 2, 3], vec![2]));
        assert_eq!(1, Solution::find_radius(vec![1, 2, 3, 4], vec![1, 4]));
        assert_eq!(3, Solution::find_radius(vec![1, 5], vec![2]));
        assert_eq!(0, Solution::find_radius(vec![1], vec![1]));
    }
}
