struct ProductOfNumbers {
    products: Vec<i32>,
    all_product: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {
    fn new() -> Self {
        Self {
            products: vec![1],
            all_product: 1,
        }
    }

    fn add(&mut self, num: i32) {
        if num == 0 {
            self.products = vec![1];
            self.all_product = 1;
        } else {
            let target = self.all_product * num;
            self.products.push(target);
            self.all_product = target;
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        if self.products.len() <= k as usize {
            0
        } else {
            self.all_product / self.products[self.products.len() - 1 - k as usize]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1352() {
        let mut obj = ProductOfNumbers::new();
        obj.add(3);
        obj.add(0);
        assert_eq!(obj.get_product(1), 0);
        assert_eq!(obj.get_product(2), 0);
        obj.add(2);
        obj.add(5);
        obj.add(4);
        assert_eq!(obj.get_product(2), 20);
        assert_eq!(obj.get_product(3), 40);
        assert_eq!(obj.get_product(4), 0);
        obj.add(8);
        assert_eq!(obj.get_product(2), 32);
    }
}
