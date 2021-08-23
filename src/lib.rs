use std::{cmp::Ordering, collections::VecDeque, fmt};

pub enum BinarySearchTree<T>
where
    T: fmt::Debug + PartialOrd,
{
    Node {
        value: T,
        left: Box<BinarySearchTree<T>>,
        right: Box<BinarySearchTree<T>>,
    },
    Empty,
}

impl<T> Default for BinarySearchTree<T>
where
    T: fmt::Debug + PartialOrd,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> BinarySearchTree<T>
where
    T: fmt::Debug + PartialOrd,
{
    pub fn new() -> Self {
        BinarySearchTree::Empty
    }

    pub fn insert(&mut self, new_value: T) {
        match self {
            BinarySearchTree::Node {
                ref value,
                ref mut left,
                ref mut right,
            } => match new_value.partial_cmp(value) {
                Some(Ordering::Less) => left.insert(new_value),
                Some(Ordering::Greater) => right.insert(new_value),
                _ => {}
            },
            BinarySearchTree::Empty => {
                *self = BinarySearchTree::Node {
                    value: new_value,
                    left: Box::new(BinarySearchTree::Empty),
                    right: Box::new(BinarySearchTree::Empty),
                };
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        match self {
            root @ BinarySearchTree::Node { .. } => {
                let mut len = 0;
                self.recursive_len(&mut len, root);
                len
            }
            BinarySearchTree::Empty => 0,
        }
    }

    fn recursive_len(&self, len: &mut usize, root: &BinarySearchTree<T>) {
        if let BinarySearchTree::Node { left, right, .. } = root {
            *len += 1;
            self.recursive_len(len, left);
            self.recursive_len(len, right);
        }
    }

    pub fn pre_order_traversal(&self) -> Option<Vec<&T>> {
        match self {
            root @ BinarySearchTree::Node { .. } => {
                let mut v = Vec::new();
                self.recursive_pre_order_traversal(&mut v, root);
                Some(v)
            }
            BinarySearchTree::Empty => None,
        }
    }

    fn recursive_pre_order_traversal<'a>(
        &self,
        v: &mut Vec<&'a T>,
        root: &'a BinarySearchTree<T>,
    ) {
        if let BinarySearchTree::Node { value, left, right } = root {
            v.push(value);
            self.recursive_pre_order_traversal(v, left);
            self.recursive_pre_order_traversal(v, right);
        }
    }

    pub fn in_order_traversal(&self) -> Option<Vec<&T>> {
        match self {
            root @ BinarySearchTree::Node { .. } => {
                let mut v = Vec::new();
                self.recursive_in_order_traversal(&mut v, root);
                Some(v)
            }
            BinarySearchTree::Empty => None,
        }
    }

    fn recursive_in_order_traversal<'a>(
        &self,
        v: &mut Vec<&'a T>,
        root: &'a BinarySearchTree<T>,
    ) {
        if let BinarySearchTree::Node { value, left, right } = root {
            self.recursive_in_order_traversal(v, left);
            v.push(value);
            self.recursive_in_order_traversal(v, right);
        }
    }

    pub fn post_order_traversal(&self) -> Option<Vec<&T>> {
        match self {
            root @ BinarySearchTree::Node { .. } => {
                let mut v = Vec::new();
                self.recursive_post_order_traversal(&mut v, root);
                Some(v)
            }
            BinarySearchTree::Empty => None,
        }
    }

    fn recursive_post_order_traversal<'a>(
        &self,
        v: &mut Vec<&'a T>,
        root: &'a BinarySearchTree<T>,
    ) {
        if let BinarySearchTree::Node { value, left, right } = root {
            self.recursive_post_order_traversal(v, left);
            self.recursive_post_order_traversal(v, right);
            v.push(value);
        }
    }

    pub fn breadth_first_traversal(&self) -> Option<Vec<&T>> {
        let mut v = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(self);

        while !queue.is_empty() {
            if let BinarySearchTree::Node { value, left, right } =
                queue.pop_front().unwrap()
            {
                v.push(value);

                if let BinarySearchTree::Node { .. } = **left {
                    queue.push_back(left);
                }

                if let BinarySearchTree::Node { .. } = **right {
                    queue.push_back(right);
                }
            }
        }

        Some(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bst_pre_order_traversal_test() {
        let mut bst = BinarySearchTree::new();
        bst.insert(60);
        bst.insert(12);
        bst.insert(90);
        bst.insert(4);
        bst.insert(1);
        bst.insert(100);
        bst.insert(37);
        bst.insert(84);
        assert_eq!(
            Some(vec![&60, &12, &4, &1, &37, &90, &84, &100]),
            bst.pre_order_traversal(),
        );
    }

    #[test]
    fn bst_in_order_traversal_test() {
        let mut bst = BinarySearchTree::new();
        bst.insert(60);
        bst.insert(12);
        bst.insert(90);
        bst.insert(4);
        bst.insert(1);
        bst.insert(100);
        bst.insert(37);
        bst.insert(84);
        assert_eq!(
            Some(vec![&1, &4, &12, &37, &60, &84, &90, &100]),
            bst.in_order_traversal(),
        );
    }

    #[test]
    fn bst_post_order_traversal_test() {
        let mut bst = BinarySearchTree::new();
        bst.insert(60);
        bst.insert(12);
        bst.insert(90);
        bst.insert(4);
        bst.insert(1);
        bst.insert(100);
        bst.insert(37);
        bst.insert(84);
        assert_eq!(
            Some(vec![&1, &4, &37, &12, &84, &100, &90, &60]),
            bst.post_order_traversal(),
        );
    }

    #[test]
    fn bst_breadth_first_traversal_test() {
        let mut bst = BinarySearchTree::new();
        bst.insert(60);
        bst.insert(12);
        bst.insert(90);
        bst.insert(4);
        bst.insert(1);
        bst.insert(100);
        bst.insert(37);
        bst.insert(84);
        assert_eq!(
            Some(vec![&60, &12, &90, &4, &37, &84, &100, &1]),
            bst.breadth_first_traversal(),
        );
    }
}
