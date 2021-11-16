pub struct Solution {}

impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let mut count = vec![0; (n * m) as usize + 1];
        let mut sum = 0;
        for i in 1..=count.len() {
            if i <= m as usize {
                for j in (i..=(i * n as usize)).step_by(i) {
                    count[j] += 1;
                }
            }
            sum += count[i];
            if k <= sum {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0668() {
        assert_eq!(Solution::find_kth_number(9895, 28045, 100787757), 3);
        assert_eq!(Solution::find_kth_number(3, 3, 5), 3);
        assert_eq!(Solution::find_kth_number(2, 3, 6), 6);
    }
}
