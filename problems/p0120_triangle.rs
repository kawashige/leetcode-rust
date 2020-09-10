pub struct Solution {}

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut sums: Vec<i32> = Vec::new();
        for r in triangle {
            let mut new_sums = Vec::new();
            if sums.len() == 0 {
                sums.push(r[0]);
                continue;
            }
            for i in 0..r.len() {
                if i == 0 {
                    new_sums.push(sums[i] + r[i]);
                } else if i == r.len() - 1 {
                    new_sums.push(sums[i - 1] + r[i]);
                } else {
                    new_sums.push(std::cmp::min(sums[i] + r[i], sums[i - 1] + r[i]));
                }
            }
            sums = new_sums;
        }
        sums.into_iter().min().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0120() {
        assert_eq!(
            11,
            Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]])
        );
        assert_eq!(
            -1,
            Solution::minimum_total(vec![vec![-1], vec![2, 3], vec![1, -1, -3]])
        );
    }
}
