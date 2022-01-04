pub struct Solution {}

impl Solution {
    pub fn longest_wpi(hours: Vec<i32>) -> i32 {
        let acc = std::iter::once(0)
            .chain(hours.into_iter())
            .scan(0, |sum, x| {
                *sum += if 8 < x { 1 } else { -1 };
                Some(*sum)
            })
            .collect::<Vec<_>>();

        (1..acc.len())
            .map(|i| i - (0..i).find(|j| acc[i] - acc[*j] > 0).unwrap_or(i))
            .max()
            .unwrap() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1124() {
        assert_eq!(Solution::longest_wpi(vec![9, 9, 6, 0, 6, 6, 9]), 3);
        assert_eq!(Solution::longest_wpi(vec![6, 6, 6]), 0);
    }
}
