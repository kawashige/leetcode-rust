pub struct Solution {}

fn mrand7() -> i32 {
    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();
    rng.gen_range(1, 8)
}

impl Solution {
    pub fn rand10() -> i32 {
        let mut n = 41;
        while n > 40 {
            let x = mrand7();
            let y = mrand7() - 1;
            n = x + 7 * y;
        }
        1 + (n - 1) % 10
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day28() {
        println!("{:?}", Solution::rand10());
    }
}
