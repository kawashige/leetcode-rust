pub struct Solution {}

impl Solution {
    pub fn min_number_of_hours(
        initial_energy: i32,
        initial_experience: i32,
        energy: Vec<i32>,
        experience: Vec<i32>,
    ) -> i32 {
        let mut hours = 0;
        let mut current_energy = initial_energy;
        let mut current_experience = initial_experience;

        for i in 0..energy.len() {
            if current_energy <= energy[i] {
                hours += energy[i] + 1 - current_energy;
                current_energy = energy[i] + 1
            }
            if current_experience <= experience[i] {
                hours += experience[i] + 1 - current_experience;
                current_experience = experience[i] + 1
            }
            current_energy -= energy[i];
            current_experience += experience[i];
        }

        hours
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2383() {
        assert_eq!(
            Solution::min_number_of_hours(5, 3, vec![1, 4, 3, 2], vec![2, 6, 3, 1]),
            8
        );
        assert_eq!(Solution::min_number_of_hours(2, 4, vec![1], vec![3]), 0);
    }
}
