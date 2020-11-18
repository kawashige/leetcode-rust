pub struct Solution {}

impl Solution {
    pub fn guessNumber(n: i32) -> i32 {
        let mut i = 0;
        let mut j = n;
        loop {
            let mid = i + (j - i) / 2;
            let result = guess(mid);
            if result == 0 {
                return mid;
            } else if result < 0 {
                j = mid - 1;
            } else {
                i = mid + 1;
            }
        }
    }
}

pub fn guess(num: i32) -> i32 {
    let pick = 50;
    if num == pick {
        0
    } else if pick < num {
        -1
    } else {
        2
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0374() {
        assert_eq!(50, Solution::guessNumber(1000));
    }
}
