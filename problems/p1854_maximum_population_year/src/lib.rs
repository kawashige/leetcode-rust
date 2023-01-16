pub struct Solution {}

impl Solution {
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        let mut counts = vec![0; 2051];
        for log in logs {
            counts[log[0] as usize] += 1;
            counts[log[1] as usize] -= 1;
        }

        let mut max_year = 0;
        let mut max = 0;

        for year in 1950..counts.len() {
            counts[year] += counts[year - 1];
            if max < counts[year] {
                max = counts[year];
                max_year = year;
            }
        }

        max_year as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1854() {
        assert_eq!(
            Solution::maximum_population(vec![vec![1993, 1999], vec![2000, 2010]]),
            1993
        );
        assert_eq!(
            Solution::maximum_population(vec![
                vec![1950, 1961],
                vec![1960, 1971],
                vec![1970, 1981]
            ]),
            1960
        );
    }
}
