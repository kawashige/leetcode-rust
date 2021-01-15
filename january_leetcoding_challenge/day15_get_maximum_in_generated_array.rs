pub struct Solution {}

impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }

        let mut generated = vec![0; n as usize + 1];
        generated[1] = 1;

        let mut max = 1;
        for i in 1..(n as usize + 1) {
            generated[i] = if i % 2 == 0 {
                generated[i / 2]
            } else {
                generated[i / 2] + generated[i / 2 + 1]
            };
            max = std::cmp::max(max, generated[i]);
        }

        max
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day15() {
        assert_eq!(1, Solution::get_maximum_generated(2));
        assert_eq!(2, Solution::get_maximum_generated(3));
    }
}
