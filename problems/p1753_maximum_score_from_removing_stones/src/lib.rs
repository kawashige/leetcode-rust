pub struct Solution {}

impl Solution {
    pub fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
        let mut score = 0;
        let mut count = [a, b, c];

        while 1 < count.iter().filter(|x| &&0 < x).count() {
            count.sort_unstable();
            count[1] -= 1;
            count[2] -= 1;
            score += 1;
        }

        score
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1753() {
        assert_eq!(Solution::maximum_score(2, 4, 6), 6);
        assert_eq!(Solution::maximum_score(4, 4, 6), 7);
        assert_eq!(Solution::maximum_score(1, 8, 8), 8);
    }
}
