pub struct Solution {}

impl Solution {
    pub fn max_distance(s: String, k: i32) -> i32 {
        let mut count = vec![0; 4];
        let mut pos = [0_i32; 2];
        let moves = [[1, 0], [-1, 0], [0, 1], [0, -1]];
        let mut result = 0;
        for i in 0..s.len() {
            let j = match s.as_bytes()[i] {
                b'N' => 0,
                b'S' => 1,
                b'E' => 2,
                b'W' => 3,
                _ => unreachable!(),
            };
            count[j] += 1;
            pos[0] += moves[j][0];
            pos[1] += moves[j][1];
            let mut c = 0;
            c += match pos[0].signum() {
                0 => count[0].max(count[1]),
                1 => count[1],
                -1 => count[0],
                _ => unreachable!(),
            };
            c += match pos[1].signum() {
                0 => count[2].max(count[3]),
                1 => count[3],
                -1 => count[2],
                _ => unreachable!(),
            };
            println!("i: {}, pos: {:?}, count: {:?}", i, pos, count);
            result = result.max(pos[0].abs() + pos[1].abs() + c.min(k) * 2);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3443() {
        assert_eq!(Solution::max_distance("NWSE".to_string(), 1), 3);
        assert_eq!(Solution::max_distance("NSWWEW".to_string(), 3), 6);
    }
}
