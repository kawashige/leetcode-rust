pub struct Solution {}

impl Solution {
    pub fn execute_instructions(n: i32, start_pos: Vec<i32>, s: String) -> Vec<i32> {
        let mut result = Vec::with_capacity(s.len());

        for i in 0..s.len() {
            let mut instructions = 0;
            let mut pos = start_pos.clone();

            for j in i..s.len() {
                pos = match s.as_bytes()[j] {
                    b'U' => vec![pos[0] - 1, pos[1]],
                    b'D' => vec![pos[0] + 1, pos[1]],
                    b'L' => vec![pos[0], pos[1] - 1],
                    b'R' => vec![pos[0], pos[1] + 1],
                    _ => unreachable!(),
                };

                if !(0..n).contains(&pos[0]) || !(0..n).contains(&pos[1]) {
                    break;
                }
                instructions += 1;
            }

            result.push(instructions);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2120() {
        assert_eq!(
            Solution::execute_instructions(3, vec![0, 1], "RRDDLU".to_string()),
            vec![1, 5, 4, 3, 1, 0]
        );
        assert_eq!(
            Solution::execute_instructions(2, vec![1, 1], "LURD".to_string()),
            vec![4, 1, 0, 0]
        );
        assert_eq!(
            Solution::execute_instructions(1, vec![0, 0], "LRUD".to_string()),
            vec![0, 0, 0, 0]
        );
    }
}
