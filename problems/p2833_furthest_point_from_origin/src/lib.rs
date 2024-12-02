pub struct Solution {}

impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let count = moves.as_bytes().iter().fold([0_i32; 3], |mut count, b| {
            match b {
                b'R' => count[0] += 1,
                b'L' => count[1] += 1,
                _ => count[2] += 1,
            }
            count
        });
        (count[0] - count[1]).abs() + count[2]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2833() {
        assert_eq!(
            Solution::furthest_distance_from_origin("L_RL__R".to_string()),
            3
        );
        assert_eq!(
            Solution::furthest_distance_from_origin("_R__LL_".to_string()),
            5
        );
        assert_eq!(
            Solution::furthest_distance_from_origin("_______".to_string()),
            7
        );
    }
}
