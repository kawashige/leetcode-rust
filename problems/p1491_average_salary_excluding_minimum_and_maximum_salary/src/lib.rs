pub struct Solution {}

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut min = std::i32::MAX;
        let mut max = std::i32::MIN;
        let mut sum = 0;

        for x in &salary {
            min = min.min(*x);
            max = max.max(*x);
            sum += x;
        }

        (sum - max - min) as f64 / (salary.len() - 2) as f64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0000() {
        assert_eq!(Solution::average(vec![4000, 3000, 1000, 2000]), 2500.00000);
        assert_eq!(Solution::average(vec![1000, 2000, 3000]), 2000.00000);
    }
}
