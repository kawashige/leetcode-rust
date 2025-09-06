pub struct Solution {}

impl Solution {
    pub fn min_operations(queries: Vec<Vec<i32>>) -> i64 {
        queries
            .into_iter()
            .map(|q| {
                let mut sum = 0;
                let mut mul = 1;
                for i in 1.. {
                    mul *= 4;
                    if mul <= q[0] {
                        continue;
                    }
                    if q[1] < mul {
                        sum += (q[1] - (mul / 4).max(q[0]) + 1) as i64 * i as i64;
                        break;
                    } else {
                        sum += (mul - (mul / 4).max(q[0])) as i64 * i as i64;
                    }
                }
                if q[0] == q[1] {
                    sum
                } else {
                    (sum + 1) / 2
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3495() {
        assert_eq!(Solution::min_operations(vec![vec![1, 2], vec![2, 4]]), 3);
        assert_eq!(Solution::min_operations(vec![vec![2, 6]]), 4);
        assert_eq!(Solution::min_operations(vec![vec![3, 10]]), 8);
    }
}
