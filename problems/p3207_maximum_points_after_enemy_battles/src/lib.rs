pub struct Solution {}

impl Solution {
    pub fn maximum_points(enemy_energies: Vec<i32>, current_energy: i32) -> i64 {
        let mut enemy_energies = enemy_energies;
        enemy_energies.sort_unstable();
        if current_energy < enemy_energies[0] {
            return 0;
        }
        let current_energy = current_energy as i64
            + (1..enemy_energies.len())
                .map(|i| enemy_energies[i] as i64)
                .sum::<i64>();
        current_energy / enemy_energies[0] as i64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3207() {
        assert_eq!(Solution::maximum_points(vec![3, 2, 2], 2), 3);
        assert_eq!(Solution::maximum_points(vec![2], 10), 5);
    }
}
