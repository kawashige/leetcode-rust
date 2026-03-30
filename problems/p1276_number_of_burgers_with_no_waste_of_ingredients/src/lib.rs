pub struct Solution {}

impl Solution {
    pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
        let x = (tomato_slices - 2 * cheese_slices) / 2;
        let y = cheese_slices - x;

        if x < 0 || y < 0 || (4 * x + 2 * y != tomato_slices) || (x + y != cheese_slices) {
            Default::default()
        } else {
            vec![x, y]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1276() {
        assert_eq!(Solution::num_of_burgers(16, 7), vec![1, 6]);
        assert_eq!(Solution::num_of_burgers(17, 4), vec![]);
        assert_eq!(Solution::num_of_burgers(4, 17), vec![]);
    }
}
