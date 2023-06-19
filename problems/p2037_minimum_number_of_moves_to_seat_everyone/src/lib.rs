pub struct Solution {}

impl Solution {
    pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
        let mut seats = seats;
        let mut students = students;
        seats.sort_unstable();
        students.sort_unstable();

        seats
            .into_iter()
            .zip(students.into_iter())
            .map(|(seat, student)| (seat - student).abs())
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2037() {
        assert_eq!(Solution::min_moves_to_seat(vec![3, 1, 5], vec![2, 7, 4]), 4);
        assert_eq!(
            Solution::min_moves_to_seat(vec![4, 1, 5, 9], vec![1, 3, 2, 6]),
            7
        );
        assert_eq!(
            Solution::min_moves_to_seat(vec![2, 2, 6, 6], vec![1, 3, 2, 6]),
            4
        );
    }
}
