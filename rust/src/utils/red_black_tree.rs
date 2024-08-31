pub struct RedBlackTree<T> {
    root: BlackNode<T>,
}

enum Node<T> {
    Black(BlackNode<T>),
    Red(RedNode<T>),
    Leaf,
}

struct BlackNode<T> {
    elem: Box<T>,
    left: Box<Node<T>>,
    right: Box<Node<T>>,
    count: usize,
    height: usize,
    black_height: usize,
}

struct RedNode<T> {
    elem: Box<T>,
    left: Box<BlackNode<T>>,
    right: Box<BlackNode<T>>,
    count: usize,
    height: usize,
    black_height: usize,
}

impl<T> RedBlackTree<T> {}
