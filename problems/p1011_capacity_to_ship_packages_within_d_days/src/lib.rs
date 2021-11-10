pub struct Solution {}

impl Solution {
    pub fn is_ok(max: i32, days: i32, weights: &Vec<i32>) -> bool {
        let mut sum = 0;
        let mut total = 1;
        for weight in weights {
            if &max < weight {
                return false;
            }
            if max < sum + weight {
                total += 1;
                sum = *weight;
            } else {
                sum += weight
            }
        }
        total <= days
    }

    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut ng = 0;
        let mut ok = weights.iter().sum::<i32>();

        while ng + 1 < ok {
            let mid = (ng + ok) / 2;
            if Self::is_ok(mid, days, &weights) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1011() {
        assert_eq!(
            Solution::ship_within_days(
                vec![
                    147, 73, 265, 305, 191, 152, 192, 293, 309, 292, 182, 157, 381, 287, 73, 162,
                    313, 366, 346, 47
                ],
                10
            ),
            602
        );
        assert_eq!(
            Solution::ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5),
            15
        );
        assert_eq!(
            Solution::ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5),
            15
        );
        assert_eq!(Solution::ship_within_days(vec![3, 2, 2, 4, 1, 4], 3), 6);
        assert_eq!(Solution::ship_within_days(vec![1, 2, 3, 1, 1], 4), 3);
    }
}
