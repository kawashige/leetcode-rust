pub struct Solution {}

impl Solution {
    pub fn is_balanced(i: i32) -> bool {
        let mut i = i;
        let mut count = [0; 10];
        while 0 < i {
            let d = i % 10;
            if d == 0 || 7 < d {
                return false;
            }
            count[d as usize] += 1;
            i /= 10;
        }
        count.iter().enumerate().all(|(i, c)| c == &0 || &i == c)
    }
    pub fn next_beautiful_number(n: i32) -> i32 {
        let mut i = n + 1;
        while !Self::is_balanced(i) {
            i += 1;
        }
        i
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2048() {
        assert_eq!(Solution::next_beautiful_number(1000000), 22);
        assert_eq!(Solution::next_beautiful_number(1), 22);
        assert_eq!(Solution::next_beautiful_number(1000), 1333);
        assert_eq!(Solution::next_beautiful_number(3000), 3133);
    }
}
