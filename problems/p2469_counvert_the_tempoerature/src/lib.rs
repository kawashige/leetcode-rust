pub struct Solution {}

impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        vec![celsius + 273.15, celsius * 1.80 + 32.0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2469() {
        assert_eq!(
            Solution::convert_temperature(36.50),
            vec![309.65000, 97.70000]
        );
        assert_eq!(
            Solution::convert_temperature(122.11),
            vec![395.26000, 251.79800]
        );
    }
}
