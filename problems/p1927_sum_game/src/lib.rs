pub struct Solution {}

impl Solution {
    pub fn sum_game(num: String) -> bool {
        let mut questions = [0; 2];
        let mut sums = [0; 2];

        for i in 0..num.len() {
            let j = (num.len() / 2 <= i) as usize;
            if num.as_bytes()[i] == b'?' {
                questions[j] += 1;
            } else {
                sums[j] += (num.as_bytes()[i] - b'0') as i32;
            }
        }

        let diff = sums[0] - sums[1];
        let min = questions[0].min(questions[1]);
        questions[0] -= min;
        questions[1] -= min;

        ((questions[0] + questions[1]) % 2 == 1)
            || (0 < questions[0] && 0 <= diff)
            || (0 < questions[1] && diff <= 0)
            || (diff.abs() != (questions[0] + questions[1]) / 2 * 9)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1927() {
        assert!(Solution::sum_game("?0?91172275656701?361205452?62??99?9??4478?7967373994600735??4?079246???5827572?81087461?089".to_string()));
        assert!(!Solution::sum_game("5023".to_string()));
        assert!(Solution::sum_game("25??".to_string()));
        assert!(!Solution::sum_game("?3295???".to_string()));
    }
}
