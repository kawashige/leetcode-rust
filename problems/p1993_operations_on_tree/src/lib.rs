struct LockingTree {
    locked: Vec<i32>,
    parent: Vec<i32>,
    children: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LockingTree {
    fn new(parent: Vec<i32>) -> Self {
        let mut children = vec![vec![]; parent.len()];
        for i in 0..parent.len() {
            if parent[i] != -1 {
                children[parent[i] as usize].push(i as i32);
            }
        }

        Self {
            locked: vec![-1; parent.len()],
            parent,
            children,
        }
    }

    fn lock(&mut self, num: i32, user: i32) -> bool {
        if self.locked[num as usize] != -1 {
            false
        } else {
            self.locked[num as usize] = user;
            true
        }
    }

    fn unlock(&mut self, num: i32, user: i32) -> bool {
        if self.locked[num as usize] != user {
            false
        } else {
            self.locked[num as usize] = -1;
            true
        }
    }

    fn upgrade(&mut self, num: i32, user: i32) -> bool {
        if self.locked[num as usize] != -1
            || self.has_locked_ancestors(num)
            || !self.unlock_children(num)
        {
            return false;
        }
        self.locked[num as usize] = user;

        true
    }

    fn has_locked_ancestors(&self, num: i32) -> bool {
        if self.locked[num as usize] != -1 {
            return true;
        }

        if self.parent[num as usize] == -1 {
            false
        } else {
            Self::has_locked_ancestors(&self, self.parent[num as usize])
        }
    }

    fn unlock_children(&mut self, num: i32) -> bool {
        let mut unlocked = false;

        if self.locked[num as usize] != -1 {
            self.locked[num as usize] = -1;
            unlocked = true;
        }

        for c in self.children[num as usize].clone() {
            unlocked |= self.unlock_children(c);
        }

        unlocked
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1993() {
        let mut obj = LockingTree::new(vec![-1, 0, 0, 1, 1, 2, 2]);
        assert!(obj.lock(2, 2));
        assert!(!obj.unlock(2, 3));
        assert!(obj.unlock(2, 2));
        assert!(obj.lock(4, 5));
        assert!(obj.upgrade(0, 1));
        assert!(!obj.lock(0, 1));
    }
}
