pub struct Solution {}

impl Solution {
    const M: usize = 1_000_000_007;
    pub fn pow(i: usize) -> usize {
        if i == 0 {
            1
        } else if i % 2 == 0 {
            let x = Self::pow(i / 2);
            x * x % Self::M
        } else {
            2 * Self::pow(i - 1) % Self::M
        }
    }

    pub fn count_ways(ranges: Vec<Vec<i32>>) -> i32 {
        let mut ranges = ranges;
        ranges.sort_unstable();

        let mut merged_ranges = vec![ranges[0].clone()];
        for i in 1..ranges.len() {
            let l = merged_ranges.len() - 1;
            if ranges[i][0] <= merged_ranges[l][1] {
                merged_ranges[l][1] = merged_ranges[l][1].max(ranges[i][1]);
            } else {
                merged_ranges.push(ranges[i].clone());
            }
        }

        Self::pow(merged_ranges.len()) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2580() {
        assert_eq!(
            Solution::count_ways(vec![
                vec![34, 56],
                vec![28, 29],
                vec![12, 16],
                vec![11, 48],
                vec![28, 54],
                vec![22, 55],
                vec![28, 41],
                vec![41, 44]
            ]),
            2
        );
        assert_eq!(Solution::count_ways(vec![vec![6, 10], vec![5, 15]]), 2);
        assert_eq!(
            Solution::count_ways(vec![vec![1, 3], vec![10, 20], vec![2, 5], vec![4, 8]]),
            4
        );
    }
}
