struct ATM {
    banknotes: Vec<i64>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ATM {
    const DOLLERS: [i64; 5] = [500, 200, 100, 50, 20];

    fn new() -> Self {
        Self {
            banknotes: vec![0; 5],
        }
    }

    fn deposit(&mut self, banknotes_count: Vec<i32>) {
        for i in 0..banknotes_count.len() {
            self.banknotes[banknotes_count.len() - 1 - i] += banknotes_count[i] as i64;
        }
    }

    fn withdraw(&mut self, amount: i32) -> Vec<i32> {
        let mut count = vec![0; 5];
        if Self::recurse(0, &mut count, amount as i64, &self.banknotes) {
            for i in 0..count.len() {
                self.banknotes[i] -= count[i] as i64;
            }
            count.into_iter().rev().collect()
        } else {
            vec![-1]
        }
    }

    fn recurse(i: usize, count: &mut Vec<i32>, remains: i64, banknotes: &[i64]) -> bool {
        if i == banknotes.len() {
            return remains == 0;
        }
        count[i] = (banknotes[i].min(remains / Self::DOLLERS[i])) as i32;
        if Self::recurse(
            i + 1,
            count,
            remains - Self::DOLLERS[i] * count[i] as i64,
            banknotes,
        ) {
            true
        } else if (i == 1 || i == 5) && 0 < count[i] {
            count[i] -= 1;
            Self::recurse(
                i + 1,
                count,
                remains - Self::DOLLERS[i] * count[i] as i64,
                banknotes,
            )
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2241() {
        let mut obj = ATM::new();
        obj.deposit(vec![0, 0, 1, 2, 1]);
        assert_eq!(obj.withdraw(600), vec![0, 0, 1, 0, 1]);
        obj.deposit(vec![0, 1, 0, 1, 1]);
        assert_eq!(obj.withdraw(600), vec![-1]);
        assert_eq!(obj.withdraw(550), vec![0, 1, 0, 0, 1]);
    }
}
