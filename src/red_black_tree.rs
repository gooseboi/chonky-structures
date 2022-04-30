use core::borrow::Borrow;
use core::cmp::Ordering;
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

    fn key_mut<'a>(&'a mut self) -> &'a mut K {
        &mut self.get_node().val.0
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

    fn right_uncle(&self) -> Self {
        self.parent().parent().right()
    }

    fn left_uncle(&self) -> Self {
        self.parent().parent().left()
    }

    fn set_right(&mut self, node: &NodePtr<K, V>) {
        unsafe { (*self.0).right = *node };
    }

    fn set_left(&mut self, node: &NodePtr<K, V>) {
        unsafe { (*self.0).left = *node };
    }

    fn set_child(&mut self, node: &NodePtr<K, V>, is_right: bool) {
        if is_right {
            self.set_left(node);
        } else {
            self.set_right(node);
        }
    }

    fn set_parent(&mut self, node: &NodePtr<K, V>) {
        unsafe { (*self.0).parent = *node };
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

    fn set(&mut self, other: &NodePtr<K, V>) {
        self.0 = other.0;
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

    fn colour(&self) -> Colour {
        self.get_node().colour
    }

    fn set_colour(&mut self, c: Colour) {
        (*self.get_node()).colour = c;
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

impl<K, V> NodePtr<K, V> {
    fn print_tree(&self)
    where
        K: core::fmt::Debug,
        V: core::fmt::Debug,
    {
        if self.is_null() {
            return;
        }
        unsafe {
            println!("Ptr    {:#?}", self.0);
            println!("Parent {:#?}", (*self.0).parent.0);
            println!("Right  {:#?}", (*self.0).right.0);
            println!("Left   {:#?}", (*self.0).left.0);
            println!("Key    {:#?}", (*self.0).val.0);
            println!("Val    {:#?}", (*self.0).val.1);
        }
        println!("");
        println!("");
        unsafe { (*self.0).right.print_tree() };
        unsafe { (*self.0).left.print_tree() };
    }
}

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
        if y.child_dir(dir).is_null() {
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
    fn correct_after_insert(&mut self, mut node: NodePtr<K, V>)
    where
        K: std::fmt::Debug,
        V: std::fmt::Debug,
    {
        while node.parent().is_red() {
            if node.parent().is_left_child() {
                if node.right_uncle().is_red() {
                    node.right_uncle().set_colour(Colour::Black);
                    node.parent().set_colour(Colour::Black);
                    node.parent().parent().set_colour(Colour::Red);
                    node = node.parent().parent();
                } else {
                    let mut parent = node.parent();
                    if node.is_right_child() {
                        self.rotate_left(node.parent());
                        let temp = parent;
                        parent = node;
                        node = temp;
                    }
                    parent.set_colour(Colour::Black);
                    parent.parent().set_colour(Colour::Red);
                    self.rotate_right(parent.parent());
                }
            } else {
                if node.left_uncle().is_red() {
                    node.parent().set_colour(Colour::Black);
                    node.left_uncle().set_colour(Colour::Black);
                    node.parent().parent().set_colour(Colour::Red);
                    node = node.parent().parent();
                } else {
                    let mut parent = node.parent();
                    if node.is_left_child() {
                        self.rotate_right(node.parent());
                        let temp = parent;
                        parent = node;
                        node = temp;
                    }
                    parent.set_colour(Colour::Black);
                    parent.parent().set_colour(Colour::Red);
                    self.rotate_left(parent.parent());
                }
            }
        }
        self.root.set_colour(Colour::Black);
    }

    /// Places into the tree, just like any normal binary search tree.
    /// If there was a new leaf node placed in the tree returns Some(NodePtr<K, V>),
    /// if the value in an existing node was replaced, returns None
    fn place_during_insert(&mut self, k: K, v: V) -> Option<NodePtr<K, V>> {
        let mut next_node = self.root;
        let mut cur_node = NodePtr::null();
        while !next_node.is_null() {
            cur_node = next_node;
            if &k > next_node.key() {
                next_node = next_node.right();
            } else if &k < next_node.key() {
                next_node = next_node.left();
            } else {
                break;
            }
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

    pub fn insert(&mut self, k: K, v: V)
    where
        K: std::fmt::Debug,
        V: std::fmt::Debug,
    {
        let ret = self.place_during_insert(k, v);
        if ret.is_none() {
            return;
        }
        let node = ret.unwrap();
        println!("Called insert");
        self.root.print_tree();
        println!("-----------------------------------------------------");
        //self.correct_after_insert(node);
        self.len += 1;
    }

    fn find_node(&self, k: &K) -> NodePtr<K, V> {
        let mut next_node = self.root;
        while !next_node.is_null() {
            match k.cmp(next_node.key()) {
                Ordering::Greater => next_node = next_node.right(),
                Ordering::Less => next_node = next_node.left(),
                Ordering::Equal => return next_node,
            }
        }
        NodePtr::null()
    }

    pub fn get<'a, Q>(&'a self, k: &Q) -> Option<&'a V>
    where
        Q: Borrow<K>,
    {
        let ret = self.find_node(k.borrow());
        if ret.is_null() {
            None
        } else {
            Some(unsafe { &(*ret.0).val.1 })
        }
    }

    pub fn iter(&self) -> Iter<K, V> {
        Iter::with_root(self.root)
    }
}

pub struct Iter<K, V> {
    cur_node: NodePtr<K, V>,
}

impl<K, V> Iter<K, V> {
    fn with_root(root: NodePtr<K, V>) -> Self {
        let mut iter = Iter { cur_node: root };
        iter.in_order_successor();
        iter
    }
    fn in_order_successor(&mut self) {
        if !self.cur_node.is_null() {
            return;
        }

        if !self.cur_node.right().is_null() {
            self.cur_node = self.cur_node.right();
            self.cur_node = self.find_leftmost()
        } else {
            self.cur_node = self.find_left_parent();
        }
    }

    fn find_left_parent(&self) -> NodePtr<K, V> {
        let mut next_node = self.cur_node;
        let mut cur_node = NodePtr::null();
        while !next_node.is_null() {
            cur_node = next_node;
            next_node = next_node.parent();
        }
        cur_node
    }

    fn find_leftmost(&self) -> NodePtr<K, V> {
        let mut next_node = self.cur_node;
        let mut cur_node = NodePtr::null();
        while !next_node.is_null() {
            cur_node = next_node;
            next_node = next_node.left();
        }
        cur_node
    }
}

#[test]
fn empty() {
    let _x: RedBlackTree<u32, u32> = RedBlackTree::new();
}

#[test]
fn insert() {
    let mut tree = RedBlackTree::new();
    tree.insert(2, 7);
    let v = tree.get(&2);
    assert_eq!(v, Some(&7));

    tree.insert(5, 3);
    let v = tree.get(&5);
    assert_eq!(v, Some(&3));

    tree.insert(4, 12);
    let v = tree.get(&4);
    assert_eq!(v, Some(&12));

    tree.insert(6, 15);
    let v = tree.get(&6);
    assert_eq!(v, Some(&15));

    tree.insert(7, 3);
    let v = tree.get(&7);
    assert_eq!(v, Some(&3));

    tree.insert(12, 23);
    let v = tree.get(&12);
    assert_eq!(v, Some(&23));
    assert_eq!(tree.len(), 6);
    assert!(false);
}
