pub struct Solution {}

impl Solution {
    pub fn count_lattice_points(circles: Vec<Vec<i32>>) -> i32 {
        let (max_x, max_y) = circles.iter().fold((0, 0), |(max_x, max_y), circle| {
            (
                max_x.max(circle[0] + circle[2]),
                max_y.max(circle[1] + circle[2]),
            )
        });

        let mut points = 0;
        for x in 0..=max_x {
            for y in 0..=max_y {
                if circles
                    .iter()
                    .find(|c| (c[0] - x) * (c[0] - x) + (c[1] - y) * (c[1] - y) <= c[2] * c[2])
                    .is_some()
                {
                    println!("{}, {}", x, y);
                    points += 1;
                }
            }
        }

        points
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2249() {
        assert_eq!(Solution::count_lattice_points(vec![vec![2, 2, 1]]), 5);
        assert_eq!(
            Solution::count_lattice_points(vec![vec![2, 2, 2], vec![3, 4, 1]]),
            16
        );
    }
}
