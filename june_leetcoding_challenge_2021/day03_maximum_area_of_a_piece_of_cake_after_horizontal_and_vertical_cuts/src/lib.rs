pub struct Solution {}

impl Solution {
    pub fn max_area(
        h: i32,
        w: i32,
        mut horizontal_cuts: Vec<i32>,
        mut vertical_cuts: Vec<i32>,
    ) -> i32 {
        horizontal_cuts.push(0);
        horizontal_cuts.push(h);
        vertical_cuts.push(0);
        vertical_cuts.push(w);
        horizontal_cuts.sort_unstable();
        vertical_cuts.sort_unstable();

        ((1..horizontal_cuts.len()).fold(0, |max, i| {
            max.max(horizontal_cuts[i] - horizontal_cuts[i - 1])
        }) as i64
            * (1..vertical_cuts.len())
                .fold(0, |max, i| max.max(vertical_cuts[i] - vertical_cuts[i - 1]))
                as i64
            % 1_000_000_007) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day03() {
        assert_eq!(Solution::max_area(5, 4, vec![1, 2, 3], vec![1, 3]), 4);
        assert_eq!(Solution::max_area(5, 4, vec![3], vec![3]), 9);
    }
}
