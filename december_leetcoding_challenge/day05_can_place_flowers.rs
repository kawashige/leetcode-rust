pub struct Solution {}

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut rest = n;
        let mut i = 0;
        while i < flowerbed.len() && 0 < rest {
            println!("i: {}, rest: {}", i, rest);
            if flowerbed[i] == 1 {
                i += 2;
            } else if (i == 0 || flowerbed[i - 1] == 0)
                && (i == flowerbed.len() - 1 || flowerbed[i + 1] == 0)
            {
                rest -= 1;
                i += 2;
            } else {
                i += 1;
            }
        }
        println!("{}", rest);
        rest == 0
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day5() {
        assert!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1));
        assert!(!Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2));
    }
}
