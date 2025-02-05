pub struct Solution {}

impl Solution {
    pub fn maximize_square_hole_area(n: i32, m: i32, h_bars: Vec<i32>, v_bars: Vec<i32>) -> i32 {
        let mut h_bars = h_bars;
        let mut v_bars = v_bars;
        h_bars.sort_unstable();
        v_bars.sort_unstable();

        let mut h_max = 1;
        let mut c = 1;
        for i in 1..h_bars.len() {
            if h_bars[i - 1] + 1 != h_bars[i] {
                h_max = h_max.max(c);
                c = 1;
            } else {
                c += 1;
            }
        }
        h_max = h_max.max(c);

        let mut v_max = 1;
        let mut c = 1;
        for i in 1..v_bars.len() {
            if v_bars[i - 1] + 1 != v_bars[i] {
                v_max = v_max.max(c);
                c = 1;
            } else {
                c += 1;
            }
        }
        v_max = v_max.max(c);

        (h_max.min(v_max) + 1) * (h_max.min(v_max) + 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2943() {
        assert_eq!(
            Solution::maximize_square_hole_area(2, 1, vec![2, 3], vec![2]),
            4
        );
        assert_eq!(
            Solution::maximize_square_hole_area(1, 1, vec![2], vec![2]),
            4
        );
        assert_eq!(
            Solution::maximize_square_hole_area(2, 3, vec![2, 3], vec![2, 4]),
            4
        );
    }
}
