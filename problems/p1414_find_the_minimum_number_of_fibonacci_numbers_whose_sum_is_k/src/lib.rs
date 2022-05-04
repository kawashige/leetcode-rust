pub struct Solution {}

impl Solution {
    pub fn find_min_fibonacci_numbers(k: i32) -> i32 {
        let mut fibonacci = Vec::with_capacity(50);
        fibonacci.push(1);
        fibonacci.push(1);
        loop {
            let x = fibonacci[fibonacci.len() - 1] + fibonacci[fibonacci.len() - 2];
            if k < x {
                break;
            }
            fibonacci.push(x);
        }

        let mut i = fibonacci.len() - 1;
        let mut target = k;
        let mut count = 0;
        while 0 < target {
            while 0 < i && target < fibonacci[i] {
                i -= 1;
            }
            target -= fibonacci[i];
            count += 1;
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1414() {
        assert_eq!(Solution::find_min_fibonacci_numbers(1000000000), 14);
        assert_eq!(Solution::find_min_fibonacci_numbers(7), 2);
        assert_eq!(Solution::find_min_fibonacci_numbers(10), 2);
        assert_eq!(Solution::find_min_fibonacci_numbers(19), 3);
    }
}
