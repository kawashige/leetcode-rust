struct Cashier {
    product_prices: [i32; 201],
    orderd: i32,
    n: i32,
    discount_rate: f64,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Cashier {
    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        let product_prices = products.into_iter().zip(prices.into_iter()).fold(
            [0; 201],
            |mut p, (product, price)| {
                p[product as usize] = price;
                p
            },
        );
        Self {
            product_prices,
            orderd: 0,
            n,
            discount_rate: (100 - discount) as f64 / 100.0,
        }
    }

    fn get_bill(&mut self, product: Vec<i32>, amount: Vec<i32>) -> f64 {
        let total_amount = product
            .into_iter()
            .zip(amount.into_iter())
            .fold(0, |total, (product, amount)| {
                total + self.product_prices[product as usize] * amount
            });

        self.orderd += 1;
        if self.orderd % self.n == 0 {
            total_amount as f64 * self.discount_rate
        } else {
            total_amount as f64
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1357() {
        let mut obj = Cashier::new(
            3,
            50,
            vec![1, 2, 3, 4, 5, 6, 7],
            vec![100, 200, 300, 400, 300, 200, 100],
        );
        assert_eq!(obj.get_bill(vec![1, 2], vec![1, 2]), 500.0);
        assert_eq!(obj.get_bill(vec![3, 7], vec![10, 10]), 4000.0);
        assert_eq!(
            obj.get_bill(vec![1, 2, 3, 4, 5, 6, 7], vec![1, 1, 1, 1, 1, 1, 1]),
            800.0
        );
        assert_eq!(obj.get_bill(vec![4], vec![10]), 4000.0);
        assert_eq!(obj.get_bill(vec![7, 3], vec![10, 10]), 4000.0);
        assert_eq!(
            obj.get_bill(vec![7, 5, 3, 1, 6, 4, 2], vec![10, 10, 10, 9, 9, 9, 7]),
            7350.0
        );
        assert_eq!(obj.get_bill(vec![2, 3, 5], vec![5, 3, 2]), 2500.0);
    }
}
