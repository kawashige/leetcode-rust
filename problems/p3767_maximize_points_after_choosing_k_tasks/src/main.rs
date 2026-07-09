pub struct Solution {}

impl Solution {
    pub fn max_points(technique1: Vec<i32>, technique2: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut technique = technique1
            .into_iter()
            .zip(technique2.into_iter())
            .collect::<Vec<_>>();
        technique.sort_unstable_by(|a, b| (b.0 - b.1).cmp(&(a.0 - a.1)));

        let mut result = 0;
        for i in 0..technique.len() {
            if i < k {
                result += technique[i].0 as i64;
            } else {
                result += (if technique[i].0 < technique[i].1 {
                    technique[i].1
                } else {
                    technique[i].0
                }) as i64;
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3767() {
        assert_eq!(Solution::max_points(vec![5, 2, 10], vec![10, 3, 8], 2), 22);
        assert_eq!(
            Solution::max_points(vec![10, 20, 30], vec![5, 15, 25], 2),
            60
        );
        assert_eq!(Solution::max_points(vec![1, 2, 3], vec![4, 5, 6], 0), 15);
    }
}

fn main() {}
