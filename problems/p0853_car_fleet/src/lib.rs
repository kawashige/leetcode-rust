pub struct Solution {}

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        if position.is_empty() {
            return 0;
        }

        let mut v = position
            .into_iter()
            .zip(speed.into_iter())
            .collect::<Vec<(i32, i32)>>();
        v.sort_unstable();

        let mut count = v.len() as i32;
        let mut max_t = (target - v[v.len() - 1].0) as f64 / v[v.len() - 1].1 as f64;
        for i in (0..(v.len() - 1)).rev() {
            let t = (target - v[i].0) as f64 / v[i].1 as f64;
            if t <= max_t {
                count -= 1;
            }
            if t > max_t {
                max_t = t;
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0853() {
        assert_eq!(Solution::car_fleet(10, vec![0, 4, 2], vec![2, 1, 3]), 1);
        assert_eq!(Solution::car_fleet(10, vec![1], vec![1]), 1);
        assert_eq!(Solution::car_fleet(10, vec![], vec![]), 0);
        assert_eq!(
            Solution::car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]),
            3
        );
    }
}
