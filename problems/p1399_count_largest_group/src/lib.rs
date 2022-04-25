pub struct Solution {}

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let (max, count) = (1..=n).fold((0, [0; 40]), |(max, mut count), mut x| {
            let mut sum = 0;
            while 0 < x {
                sum += x % 10;
                x /= 10;
            }
            count[sum as usize] += 1;
            (max.max(count[sum as usize]), count)
        });
        count.into_iter().filter(|c| c == &max).count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1399() {
        assert_eq!(Solution::count_largest_group(13), 4);
        assert_eq!(Solution::count_largest_group(2), 2);
    }
}
