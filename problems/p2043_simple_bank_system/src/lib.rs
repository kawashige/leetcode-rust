struct Bank {
    balance: Vec<i64>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Bank {
    fn new(balance: Vec<i64>) -> Self {
        Self { balance }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        if (1..=self.balance.len()).contains(&(account1 as usize))
            && (1..=self.balance.len()).contains(&(account2 as usize))
            && money <= self.balance[account1 as usize - 1]
        {
            self.balance[account1 as usize - 1] -= money;
            self.balance[account2 as usize - 1] += money;
            true
        } else {
            false
        }
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        if (1..=self.balance.len()).contains(&(account as usize)) {
            self.balance[account as usize - 1] += money;
            true
        } else {
            false
        }
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        if (1..=self.balance.len()).contains(&(account as usize))
            && money <= self.balance[account as usize - 1]
        {
            self.balance[account as usize - 1] -= money;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2043() {
        let mut obj = Bank::new(vec![10, 100, 20, 50, 30]);
        assert!(obj.withdraw(3, 10));
        assert!(obj.transfer(5, 1, 20));
        assert!(obj.deposit(5, 20));
        assert!(!obj.transfer(3, 4, 15));
        assert!(!obj.withdraw(10, 50));
    }
}
