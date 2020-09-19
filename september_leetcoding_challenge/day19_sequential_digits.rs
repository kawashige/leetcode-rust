pub struct Solution {}

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let l_digits = low.to_string().len();
        let h_digits = high.to_string().len();

        let mut results = Vec::new();
        for j in l_digits..=h_digits {
            for i in 1..(11 - j) {
                let num = (i..10)
                    .take(j)
                    .map(|i| i.to_string())
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap();
                if low <= num && num <= high {
                    results.push(num);
                }
            }
        }
        results
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day19() {
        assert_eq!(vec![123, 234], Solution::sequential_digits(100, 300));
        assert_eq!(
            vec![1234, 2345, 3456, 4567, 5678, 6789, 12345],
            Solution::sequential_digits(1000, 13000)
        );
    }
}
