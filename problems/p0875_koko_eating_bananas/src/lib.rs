pub struct Solution {}

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut ng = 0;
        let mut ok = *piles.iter().max().unwrap() as i64;

        while ng + 1 < ok {
            let mid = (ng + ok) / 2;
            if piles
                .iter()
                .map(|x| (*x as i64 + mid - 1) / mid)
                .sum::<i64>()
                <= h as i64
            {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0875() {
        assert_eq!(
            Solution::min_eating_speed(vec![805306368, 805306368, 805306368], 1000000000),
            3
        );
        assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 8), 4);
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
    }
}
