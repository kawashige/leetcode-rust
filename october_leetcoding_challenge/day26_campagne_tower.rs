pub struct Solution {}

impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut tower = Vec::new();
        for i in 1..(query_row as usize + 3) {
            tower.push(vec![0.0; i]);
        }

        tower[0][0] = poured as f64;
        for i in 0..(tower.len() - 1) {
            let mut overflowed = false;
            for j in 0..tower[i].len() {
                if 1.0 < tower[i][j] {
                    overflowed = true;
                    let overflow = tower[i][j] - 1.0;
                    tower[i][j] = 1.0;
                    tower[i + 1][j] += overflow / 2.0;
                    tower[i + 1][j + 1] += overflow / 2.0;
                }
            }
            if !overflowed {
                break;
            }
        }

        tower[query_row as usize][query_glass as usize]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_26() {
        assert_eq!(0.18750, Solution::champagne_tower(25, 6, 1));
        assert_eq!(0.00000, Solution::champagne_tower(0, 1, 1));
        assert_eq!(0.00000, Solution::champagne_tower(0, 0, 0));
        assert_eq!(0.00000, Solution::champagne_tower(1, 1, 1));
        assert_eq!(0.50000, Solution::champagne_tower(2, 1, 1));
        assert_eq!(1.00000, Solution::champagne_tower(100000009, 33, 17));
    }
}
