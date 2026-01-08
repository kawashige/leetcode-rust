pub struct Solution {}

impl Solution {
    const M: u128 = 1_000_000;

    pub fn is_ok(mid: u128, squares: &Vec<Vec<u128>>) -> bool {
        let mut upper = 0;
        let mut lower = 0;

        for i in 0..squares.len() {
            if squares[i][1] + squares[i][2] <= mid {
                lower += squares[i][2] * squares[i][2];
            } else if mid <= squares[i][1] {
                upper += squares[i][2] * squares[i][2];
            } else {
                upper += (squares[i][1] + squares[i][2] - mid) * squares[i][2];
                lower += (mid - squares[i][1]) * squares[i][2]
            }
        }

        upper <= lower
    }

    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let squares = squares
            .into_iter()
            .map(|s| {
                vec![
                    s[0] as u128 * Self::M,
                    s[1] as u128 * Self::M,
                    s[2] as u128 * Self::M,
                ]
            })
            .collect::<Vec<_>>();
        let max_y = squares.iter().map(|s| s[1] + s[2]).max().unwrap();

        let mut ok = max_y;
        let mut ng = 0;

        while ng + 1 < ok {
            let mid = (ok + ng) / 2;
            if Self::is_ok(mid, &squares) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok as f64 / Self::M as f64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3453() {
        assert_eq!(
            Solution::separate_squares(vec![
                vec![522261215, 954313664, 225462],
                vec![628661372, 718610752, 10667],
                vec![619734768, 941310679, 44788],
                vec![352367502, 656774918, 289036],
                vec![860247066, 905800565, 100123],
                vec![817623994, 962847576, 71460],
                vec![691552058, 782740602, 36271],
                vec![911356, 152015365, 513881],
                vec![462847044, 859151855, 233567],
                vec![672324240, 954509294, 685569]
            ]),
            954521423.80202
        );
        assert_eq!(
            Solution::separate_squares(vec![vec![0, 0, 1], vec![2, 2, 1]]),
            1.00000
        );
        assert_eq!(
            Solution::separate_squares(vec![vec![0, 0, 2], vec![1, 1, 1]]),
            1.16667
        );
    }
}
