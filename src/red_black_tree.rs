use core::borrow::Borrow;
use core::cmp::Ordering;
use core::marker::PhantomData;
use core::ptr;

struct NodePtr<K, V>(*mut Node<K, V>);

impl<K, V> NodePtr<K, V> {
    fn null() -> Self {
        NodePtr(ptr::null_mut())
    }

    fn is_null(&self) -> bool {
        self.0.is_null()
    }

    fn get_node(&self) -> &mut Node<K, V> {
        assert!(!self.is_null());
        unsafe { &mut (*self.0) }
    }

    fn key<'a>(&'a self) -> &'a K {
        &self.get_node().val.0
    }

    fn val_mut<'a>(&'a mut self) -> &'a mut V {
        &mut self.get_node().val.1
    }

    fn right(&self) -> Self {
        self.get_node().right
    }

    fn left(&self) -> Self {
        self.get_node().left
    }

    fn child_dir(&self, is_right: bool) -> Self {
        let ret = self.get_node();
        if is_right {
            ret.right
        } else {
            ret.left
        }
    }

    fn parent(&self) -> Self {
        self.get_node().parent
    }

    fn uncle(&self) -> Self {
        if self.parent().is_right_child() {
            self.parent().parent().left()
        } else {
            self.parent().parent().right()
        }
    }

    fn set_right(&mut self, node: &NodePtr<K, V>) {
        self.get_node().right = *node;
    }

    fn set_left(&mut self, node: &NodePtr<K, V>) {
        self.get_node().left = *node;
    }

    fn set_child(&mut self, node: &NodePtr<K, V>, is_right: bool) {
        if is_right {
            self.set_right(node);
        } else {
            self.set_left(node);
        }
    }

    fn set_parent(&mut self, node: &NodePtr<K, V>) {
        self.get_node().parent = *node;
    }

    fn is_left_child(&self) -> bool {
        if self.parent().is_null() {
            false
        } else {
            self.parent().left() == *self
        }
    }

    fn is_right_child(&self) -> bool {
        if self.parent().is_null() {
            false
        } else {
            self.parent().right() == *self
        }
    }

    fn is_child_dir(&self, is_right: bool) -> bool {
        if is_right {
            self.is_right_child()
        } else {
            self.is_left_child()
        }
    }

    fn set_colour(&mut self, c: Colour) {
        (*self.get_node()).colour = c;
    }

    fn is_black(&self) -> bool {
        if self.is_null() {
            true
        } else {
            self.get_node().colour == Colour::Black
        }
    }

    fn is_red(&self) -> bool {
        !self.is_black()
    }
}

impl<K, V> Clone for NodePtr<K, V> {
    fn clone(&self) -> Self {
        NodePtr(self.0)
    }
}

impl<K, V> Copy for NodePtr<K, V> {}

impl<K, V> PartialEq for NodePtr<K, V> {
    fn eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0
    }
}
impl<K, V> Eq for NodePtr<K, V> {}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Colour {
    Red,
    Black,
}

struct Node<K, V> {
    right: NodePtr<K, V>,
    left: NodePtr<K, V>,
    parent: NodePtr<K, V>,
    colour: Colour,
    val: (K, V),
}

impl<K, V> Node<K, V> {
    fn new(k: K, v: V) -> Self {
        Self {
            right: NodePtr::null(),
            left: NodePtr::null(),
            parent: NodePtr::null(),
            colour: Colour::Red,
            val: (k, v),
        }
    }
}

pub struct RedBlackTree<K, V> {
    root: NodePtr<K, V>,
    len: usize,
}

impl<K, V> RedBlackTree<K, V> {
    pub fn new() -> Self {
        Self {
            root: NodePtr::null(),
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    fn clear_node(node: NodePtr<K, V>) {
        if node.is_null() {
            return;
        }

        unsafe {
            Self::clear_node(node.right());
            Self::clear_node(node.left());
            Box::from_raw(node.0);
        }
    }

    fn clear(&mut self) {
        Self::clear_node(self.root);
    }
}

impl<K, V> Default for RedBlackTree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> RedBlackTree<K, V>
where
    K: Ord,
{
    /// Rotates the subtree starting at `node` in the given direction,
    /// true representing a right rotation and false a left rotation
    fn rotate_dir(&mut self, mut node: NodePtr<K, V>, dir: bool) {
        let mut y = node.child_dir(!dir);
        node.set_child(&y.child_dir(dir), !dir);
        if !y.child_dir(dir).is_null() {
            y.child_dir(dir).set_parent(&node);
        }
        y.set_parent(&node.parent());
        if node.parent().is_null() {
            self.root = y;
        } else if node.is_child_dir(dir) {
            node.parent().set_child(&y, dir);
        } else {
            node.parent().set_child(&y, !dir);
        }
        y.set_child(&node, dir);
        node.set_parent(&y);
    }

    /// Rotates the subtree starting at `node` to the left
    fn rotate_left(&mut self, node: NodePtr<K, V>) {
        self.rotate_dir(node, false);
    }

    /// Rotates the subtree starting at `node` to the right
    fn rotate_right(&mut self, node: NodePtr<K, V>) {
        self.rotate_dir(node, true);
    }

    /// Performs the necessary corrections to the tree to fit the 4
    /// Red Black Tree criteria.
    fn correct_after_insert(&mut self, mut node: NodePtr<K, V>) {
        while node.parent().is_red() {
            if node.parent().is_left_child() {
                if node.uncle().is_red() {
                    node.uncle().set_colour(Colour::Black);
                    node.parent().set_colour(Colour::Black);
                    node.parent().parent().set_colour(Colour::Red);
                    node = node.parent().parent();
                } else {
                    if node.is_right_child() {
                        node = node.parent();
                        self.rotate_left(node);
                    }
                    node.parent().set_colour(Colour::Black);
                    node.parent().parent().set_colour(Colour::Red);
                    self.rotate_right(node.parent().parent());
                }
            } else {
                if node.uncle().is_red() {
                    node.parent().set_colour(Colour::Black);
                    node.uncle().set_colour(Colour::Black);
                    node.parent().parent().set_colour(Colour::Red);
                    node = node.parent().parent();
                } else {
                    if node.is_left_child() {
                        node = node.parent();
                        self.rotate_right(node);
                    }
                    node.parent().set_colour(Colour::Black);
                    node.parent().parent().set_colour(Colour::Red);
                    self.rotate_left(node.parent().parent());
                }
            }
        }
        self.root.set_colour(Colour::Black);
    }

    /// Places into the tree, just like any normal binary search tree.
    /// If there was a new leaf node placed in the tree returns Some(NodePtr<K, V>),
    /// if the value in an existing node was replaced, returns None
    fn place(&mut self, k: K, v: V) -> Option<NodePtr<K, V>> {
        let mut next_node = self.root;
        let mut cur_node = NodePtr::null();
        while !next_node.is_null() {
            cur_node = next_node;
            match k.cmp(next_node.key()) {
                Ordering::Less => next_node = next_node.left(),
                Ordering::Greater => next_node = next_node.right(),
                Ordering::Equal => break,
            };
        }

        if self.root.is_null() {
            let node = Box::new(Node::new(k, v));
            let node = NodePtr(Box::leak(node));
            self.root = node;
            Some(self.root)
        } else {
            match k.cmp(cur_node.key()) {
                Ordering::Less => {
                    let node = Box::new(Node::new(k, v));
                    let mut node = NodePtr(Box::leak(node));
                    cur_node.set_left(&node);
                    node.set_parent(&cur_node);
                    Some(node)
                }
                Ordering::Greater => {
                    let node = Box::new(Node::new(k, v));
                    let mut node = NodePtr(Box::leak(node));
                    cur_node.set_right(&node);
                    node.set_parent(&cur_node);
                    Some(node)
                }
                Ordering::Equal => {
                    *cur_node.val_mut() = v;
                    None
                }
            }
        }
    }

    pub fn insert(&mut self, k: K, v: V) {
        let ret = self.place(k, v);
        if ret.is_none() {
            return;
        }
        let node = ret.unwrap();
        self.correct_after_insert(node);
        self.len += 1;
    }

    pub fn remove<Q>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let node = self.find_node(k);
        self.delete(node);
        self.len -= 1;
        None
    }

    fn delete<Q>(&mut self, node: NodePtr<K, V>)
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        unimplemented!()
    }

    fn find_node<Q>(&self, k: &Q) -> NodePtr<K, V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let mut next_node = self.root;
        while !next_node.is_null() {
            match k.cmp(next_node.key().borrow()) {
                Ordering::Greater => next_node = next_node.right(),
                Ordering::Less => next_node = next_node.left(),
                Ordering::Equal => return next_node,
            }
        }
        NodePtr::null()
    }

    pub fn get<'a, Q>(&'a self, k: &Q) -> Option<&'a V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let ret = self.find_node(k);
        if ret.is_null() {
            None
        } else {
            Some(unsafe { &(*ret.0).val.1 })
        }
    }

    pub fn get_mut<'a, Q>(&'a mut self, k: &Q) -> Option<&'a mut V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let ret = self.find_node(k);
        if ret.is_null() {
            None
        } else {
            Some(unsafe { &mut (*ret.0).val.1 })
        }
    }

    pub fn contains_key<Q>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.get(k).is_some()
    }

    pub fn iter(&self) -> Iter<K, V> {
        Iter::new(self.root, self.root, self.len)
    }

    pub fn values(&self) -> Values<K, V> {
        let iter = self.iter();
        Values {
            iter,
            _marker: PhantomData,
        }
    }

    pub fn keys(&self) -> Keys<K, V> {
        let iter = self.iter();
        Keys {
            iter,
            _marker: PhantomData,
        }
    }
}

impl<K, V> Drop for RedBlackTree<K, V> {
    fn drop(&mut self) {
        self.clear();
    }
}

#[derive(Copy, Clone)]
pub struct Iter<'a, K: 'a, V: 'a> {
    head: NodePtr<K, V>,
    tail: NodePtr<K, V>,
    remaining: usize,
    _marker: PhantomData<&'a (K, V)>,
}

impl<K, V> Iter<'_, K, V> {
    fn new(head: NodePtr<K, V>, tail: NodePtr<K, V>, len: usize) -> Self {
        let mut iter = Iter {
            head,
            tail,
            remaining: len,
            _marker: PhantomData,
        };
        iter.head = iter.find_leftmost();
        iter
    }

    fn in_order_next(&mut self) {
        if self.head.is_null() {
            return;
        }

        if !self.head.right().is_null() {
            self.head = self.head.right();
            self.head = self.find_leftmost()
        } else {
            self.head = self.find_next_parent();
        }
    }

    fn find_next_parent(&self) -> NodePtr<K, V> {
        let mut next_node = self.head;
        while !next_node.is_null() {
            if next_node.is_left_child() {
                return next_node.parent();
            } else {
                next_node = next_node.parent();
            }
        }
        next_node
    }

    fn find_leftmost(&self) -> NodePtr<K, V> {
        let mut next_node = self.head;
        let mut cur_node = NodePtr::null();
        while !next_node.is_null() {
            cur_node = next_node;
            next_node = next_node.left();
        }
        cur_node
    }

    fn in_order_prev(&mut self) {
        if self.tail.is_null() {
            return;
        }

        if !self.tail.left().is_null() {
            self.tail = self.tail.left();
            self.tail = self.find_rightmost()
        } else {
            self.tail = self.find_prev_parent();
        }
    }

    fn find_prev_parent(&self) -> NodePtr<K, V> {
        let mut next_node = self.tail;
        while !next_node.is_null() {
            if next_node.is_right_child() {
                return next_node.parent();
            } else {
                next_node = next_node.parent();
            }
        }
        next_node
    }

    fn find_rightmost(&self) -> NodePtr<K, V> {
        let mut next_node = self.tail;
        let mut cur_node = NodePtr::null();
        while !next_node.is_null() {
            cur_node = next_node;
            next_node = next_node.right();
        }
        cur_node
    }
}

impl<'a, K: 'a, V: 'a> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.head.is_null() || self.remaining == 0 {
            None
        } else {
            let p = unsafe { (&(*self.head.0).val.0, &(*self.head.0).val.1) };
            self.in_order_next();
            self.remaining -= 1;
            Some(p)
        }
    }
}

impl<'a, K: 'a, V: 'a> DoubleEndedIterator for Iter<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.tail.is_null() || self.remaining == 0 {
            None
        } else {
            let p = unsafe { (&(*self.head.0).val.0, &(*self.head.0).val.1) };
            self.in_order_prev();
            self.remaining -= 1;
            Some(p)
        }
    }
}
#[derive(Copy, Clone)]
pub struct Keys<'a, K: 'a, V: 'a> {
    iter: Iter<'a, K, V>,
    _marker: PhantomData<&'a (K, V)>,
}

impl<'a, K: 'a, V: 'a> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(k, _)| k)
    }
}

impl<'a, K: 'a, V: 'a> DoubleEndedIterator for Keys<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|(k, _)| k)
    }
}

#[derive(Copy, Clone)]
pub struct Values<'a, K: 'a, V: 'a> {
    iter: Iter<'a, K, V>,
    _marker: PhantomData<&'a (K, V)>,
}

impl<'a, K: 'a, V: 'a> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(_, v)| v)
    }
}

impl<'a, K: 'a, V: 'a> DoubleEndedIterator for Values<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|(_, v)| v)
    }
}

mod tests {
    use super::{Iter, RedBlackTree};
    #[test]
    fn empty() {
        let _x: RedBlackTree<u32, u32> = RedBlackTree::new();
    }

    #[test]
    fn insert() {
        let mut tree = RedBlackTree::new();

        tree.insert(15, 7);
        tree.insert(2, 7);
        tree.insert(5, 3);
        tree.insert(12, 23);
        tree.insert(12, 25);
        tree.insert(4, 12);
        tree.insert(6, 15);
        tree.insert(7, 3);
        tree.insert(3, 21);
        tree.insert(9, 91);
        assert_eq!(tree.len(), 9);

        let iter = tree.keys();
        let vec: Vec<_> = iter.collect();
        assert_eq!(vec, [&2, &3, &4, &5, &6, &7, &9, &12, &15]);
    }
}

#[test]
fn test_lots_of_insertions() {
    let mut m = RedBlackTree::new();

    // Try this a few times to make sure we never screw up the hashmap's
    // internal state.
    for _ in 0..10 {
        assert!(m.is_empty());

        for i in 1..101 {
            m.insert(i, i);

            for j in 1..i + 1 {
                let r = m.get(&j);
                assert_eq!(r, Some(&j));
            }

            for j in i + 1..101 {
                let r = m.get(&j);
                assert_eq!(r, None);
            }
        }

        for i in 101..201 {
            assert!(!m.contains_key(&i));
        }

        // remove forwards
        for i in 1..101 {
            assert!(m.remove(&i).is_some());

            for j in 1..i + 1 {
                assert!(!m.contains_key(&j));
            }

            for j in i + 1..101 {
                assert!(m.contains_key(&j));
            }
        }

        for i in 1..101 {
            assert!(!m.contains_key(&i));
        }

        for i in 1..101 {
            m.insert(i, i);
        }

        // remove backwards
        for i in (1..101).rev() {
            assert!(m.remove(&i).is_some());

            for j in i..101 {
                assert!(!m.contains_key(&j));
            }

            for j in 1..i {
                assert!(m.contains_key(&j));
            }
        }
    }
}
