pub struct Solution {}

impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        if n < 10 {
            return n;
        }

        let mut i = 1;
        let mut j = 9;
        let mut sum: i64 = j * i;
        while sum + (i + 1) * (j * 10) < n as i64 {
            i += 1;
            j *= 10;
            sum += i * j;
        }

        let mut k = ((n - sum as i32) % (i as i32 + 1)) as usize;
        let mut num = (10 as i32).pow(i as u32) + (n - sum as i32) / (i as i32 + 1);
        if k == 0 {
            num -= 1;
            k = i as usize + 1;
        }

        println!("i: {}, j: {}, sum: {}, num: {}, k: {}", i, j, sum, num, k);

        num.to_string()
            .chars()
            .nth(k - 1)
            .unwrap()
            .to_digit(10)
            .unwrap() as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0400() {
        assert_eq!(1, Solution::find_nth_digit(1000000000));
        assert_eq!(3, Solution::find_nth_digit(3));
        assert_eq!(0, Solution::find_nth_digit(11));
        assert_eq!(3, Solution::find_nth_digit(201));
        assert_eq!(9, Solution::find_nth_digit(189));
        assert_eq!(9, Solution::find_nth_digit(188));
        assert_eq!(4, Solution::find_nth_digit(2909));
        assert_eq!(1, Solution::find_nth_digit(2906));
    }
}
