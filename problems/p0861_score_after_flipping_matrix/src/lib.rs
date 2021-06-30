pub struct Solution {}

impl Solution {
    pub fn matrix_score(grid: Vec<Vec<i32>>) -> i32 {
        let nums = grid
            .iter()
            .map(|row| row.iter().fold(0, |acc, i| acc * 2 + *i as usize))
            .collect::<Vec<usize>>();

        let flip = std::iter::repeat(1)
            .take(grid[0].len())
            .fold(0, |acc, i| acc * 2 + i as usize);

        let mut max = 0;

        for n in 0..2_usize.pow(grid[0].len() as u32) {
            let mut sum = 0;
            for i in 0..nums.len() {
                sum += std::cmp::max(n ^ nums[i], n ^ nums[i] ^ flip);
            }
            max = std::cmp::max(max, sum);
        }

        max as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0861() {
        assert_eq!(
            Solution::matrix_score(vec![vec![0, 0, 1, 1], vec![1, 0, 1, 0], vec![1, 1, 0, 0]]),
            39
        );
    }
}
