pub struct Solution {}

impl Solution {
    pub fn recurse(
        left_top: (usize, usize),
        right_bottom: (usize, usize),
        horizontal_cut: &[i32],
        vertical_cut: &[i32],
    ) -> i32 {
        let mut cost = 0;
        let h = (left_top.0 + 1..right_bottom.0)
            .max_by_key(|i| horizontal_cut[i - 1])
            .unwrap_or(0);
        let v = (left_top.1 + 1..right_bottom.1)
            .max_by_key(|j| vertical_cut[j - 1])
            .unwrap_or(0);

        if h == 0 && v == 0 {
            return 0;
        }

        if v == 0 || (h != 0 && vertical_cut[v - 1] < horizontal_cut[h - 1]) {
            cost += horizontal_cut[h - 1];
            cost += Self::recurse(left_top, (h, right_bottom.1), horizontal_cut, vertical_cut);
            cost += Self::recurse((h, left_top.1), right_bottom, horizontal_cut, vertical_cut);
        } else {
            cost += vertical_cut[v - 1];
            cost += Self::recurse(left_top, (right_bottom.0, v), horizontal_cut, vertical_cut);
            cost += Self::recurse((left_top.0, v), right_bottom, horizontal_cut, vertical_cut);
        }
        cost
    }

    pub fn minimum_cost(m: i32, n: i32, horizontal_cut: Vec<i32>, vertical_cut: Vec<i32>) -> i32 {
        Self::recurse(
            (0, 0),
            (m as usize, n as usize),
            &horizontal_cut,
            &vertical_cut,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3218() {
        assert_eq!(Solution::minimum_cost(3, 2, vec![1, 3], vec![5]), 13);
        assert_eq!(Solution::minimum_cost(2, 2, vec![7], vec![4]), 15);
    }
}
