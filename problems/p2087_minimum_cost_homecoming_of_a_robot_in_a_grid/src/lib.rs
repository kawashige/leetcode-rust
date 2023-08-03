pub struct Solution {}

impl Solution {
    pub fn min_cost(
        start_pos: Vec<i32>,
        home_pos: Vec<i32>,
        row_costs: Vec<i32>,
        col_costs: Vec<i32>,
    ) -> i32 {
        (if start_pos[0] < home_pos[0] {
            start_pos[0] + 1..home_pos[0] + 1
        } else {
            home_pos[0]..start_pos[0]
        })
        .map(|r| row_costs[r as usize])
        .sum::<i32>()
            + (if start_pos[1] < home_pos[1] {
                start_pos[1] + 1..home_pos[1] + 1
            } else {
                home_pos[1]..start_pos[1]
            })
            .map(|c| col_costs[c as usize])
            .sum::<i32>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2087() {
        assert_eq!(
            Solution::min_cost(vec![1, 0], vec![2, 3], vec![5, 4, 3], vec![8, 2, 6, 7]),
            18
        );
        assert_eq!(
            Solution::min_cost(vec![0, 0], vec![0, 0], vec![5], vec![26]),
            0
        );
    }
}
