pub struct Solution {}

impl Solution {
    pub fn longest_mountain(a: Vec<i32>) -> i32 {
        if a.len() < 3 {
            return 0;
        }

        let mut result = 0;
        let mut up_phase = true;
        let mut prev_value = a[0];
        let mut length = 1;
        for i in 1..a.len() {
            if up_phase {
                if prev_value < a[i] {
                    length += 1;
                } else if a[i] < prev_value && 1 < length {
                    up_phase = false;
                    length += 1;
                } else {
                    length = 1;
                }
            } else {
                if a[i] < prev_value {
                    length += 1;
                } else {
                    result = std::cmp::max(result, length);
                    length = if prev_value < a[i] { 2 } else { 1 };
                    up_phase = true;
                }
            }
            prev_value = a[i];
        }
        if !up_phase && 2 < length {
            std::cmp::max(result, length)
        } else {
            result
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day16() {
        assert_eq!(
            4,
            Solution::longest_mountain(vec![875, 884, 239, 731, 723, 685])
        );
        assert_eq!(6, Solution::longest_mountain(vec![2, 1, 4, 7, 3, 2, 1]));
        assert_eq!(5, Solution::longest_mountain(vec![2, 1, 4, 7, 3, 2, 5]));
        assert_eq!(0, Solution::longest_mountain(vec![2, 2, 2]));
    }
}
