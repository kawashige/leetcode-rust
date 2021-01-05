pub struct Solution {}

impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let area_sqrt = (area as f32).sqrt() as i32;
        let w = (1..=area_sqrt)
            .filter(|i| area % i == 0)
            .min_by_key(|i| area / i - i)
            .unwrap();
        vec![area / w, w]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0492() {
        assert_eq!(vec![1, 1], Solution::construct_rectangle(1));
        assert_eq!(vec![2, 2], Solution::construct_rectangle(4));
        assert_eq!(vec![37, 1], Solution::construct_rectangle(37));
        assert_eq!(vec![427, 286], Solution::construct_rectangle(122122));
    }
}
