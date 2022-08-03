use core::panic;
use std::cmp::max;
use std::fmt::Display;
use std::mem;

#[derive(Debug, Clone)]
pub enum AvlTree<T> {
    Leaf,
    Node {
        elem: Box<T>,
        left: Box<AvlTree<T>>,
        right: Box<AvlTree<T>>,
        height: usize,
        count: usize,
    },
}

impl<T> AvlTree<T> 
where 
    T: Ord
{
    pub fn new() -> Self {
        AvlTree::Leaf
    }

    pub fn from_t(elem: T) -> Self {
        AvlTree::Node { 
            elem: Box::new(elem), 
            left: Box::new(AvlTree::new()), 
            right: Box::new(AvlTree::new()), 
            height: 1, count: 1 
        }
    }

    fn height(&self)->usize{
        match self {
            Self::Leaf => 0,
            Self::Node{height, ..} => *height
        }
    }
    fn count(&self)->usize{
        match self {
            Self::Leaf => 0,
            Self::Node{count, ..} => *count
        }
    }

    pub fn insert(&mut self, new_elem: T) -> (usize, bool) {
        let (index, inserted);
        match self {
            Self::Leaf => {
                *self = AvlTree::from_t(new_elem);
                return (0, true);
            },
            Self::Node { elem, left, right, count, height} => {
                if new_elem < **elem {
                    (index, inserted) = left.insert(new_elem);
                } else if new_elem > **elem {
                    (index, inserted) = {|(i,b)| (i+left.count()+1, b)}(right.insert(new_elem));
                } else {
                    return (left.count(), false);
                }
                *count = left.count() + right.count() + 1;
                *height = max(left.height(), right.height()) + 1;
            }
        }
        self.balance();
        return (index, inserted);
    }

    fn balance(&mut self){
        match self {
            Self::Leaf => (),
            Self::Node { left:left_n, right:right_n, ..} => {

                if left_n.height() > right_n.height()+1 {
                    match &**left_n {
                        Self::Leaf => panic!("In balancing the left child was to high, but it was a Leaf"),
                        Self::Node { left:left_l, right:right_l, ..} => {
                            if left_l.height() >= right_l.height() {
                                self.rotate_right()
                            } else {
                                left_n.rotate_left();
                                self.rotate_right();
                            }
                        }
                    };
                } else if right_n.height() > left_n.height()+1{
                    match &**right_n {
                        Self::Leaf => panic!("In balancing the left child was to high, but it was a Leaf"),
                        Self::Node { left:left_r, right:right_r, ..} => {
                            if right_r.height() >= left_r.height() {
                                self.rotate_left();
                            } else {
                                right_n.rotate_right();
                                self.rotate_left();
                            }
                        }
                    }
                }
            }
        }
    }
    fn rotate_right(&mut self){
        match self {
            Self::Leaf => panic!("can't rotate a Leaf to the right"),
            Self::Node { elem:elem_n, left:left_n, right:right_n, height:height_n, count:count_n } => {
                let a;
                let b;
                let c;
                match &mut **left_n {
                    Self::Leaf => panic!("can't rotate a node with left leaf to the right"),
                    Self::Node { elem:elem_l, left:left_l, right:right_l, height:height_l, count:count_l } => {
                        //extracting all relevant subtrees owned, and replacing them with Leafs
                        a = mem::take(left_l);
                        b = mem::take(right_l);
                        c = mem::take(right_n);

                        // creating the new right child of the root that is to be rotated. It is created in place at the location of the former left child, so no new allocations have to be done
                        mem::swap(elem_n, elem_l);
                        *left_l = b;
                        *right_l = c;
                        // recalculating hight and count of the new right child
                        *height_l = max(left_l.height(), right_l.height()) + 1;
                        *count_l = left_l.count() + right_l.count() + 1;
                    }
                }

                *right_n = a;
                mem::swap(right_n, left_n);
                // replacing the left child of the root with "a" but extracting the olf left child as owned. Here we generated the new right child so we now can insert it at this location
                // recalculating hight and count of the root
                *height_n = max(left_n.height(), right_n.height()) + 1;
                *count_n = left_n.count() + right_n.count() + 1;
            }
        }
    }
    
     fn rotate_left(&mut self){
        match self {
            Self::Leaf => panic!("can't rotate a Leaf to the right"),
            Self::Node { elem:elem_n, left:left_n, right:right_n, height:height_n, count:count_n } => {
                let a;
                let b;
                let c;
                match &mut **right_n {
                    Self::Leaf => panic!("can't rotate a node with left leaf to the right"),
                    Self::Node { elem:elem_r, left:left_r, right:right_r, height:height_r, count:count_r } => {
                        //extracting all relevant subtrees owned, and replacing them with Leafs
                        a = mem::take(left_n );
                        b = mem::take(left_r );
                        c = mem::take(right_r);
                        // creating the new right child of the root that is to be rotated. It is created in place at the location of the former left child, so no new allocations have to be done
                        mem::swap(elem_n, elem_r);
                        *left_r = a;
                        *right_r = b;
                        // recalculating hight and count of the new left child
                        *height_r = max(left_r.height(), right_r.height()) + 1;
                        *count_r = left_r.count() + right_r.count() + 1;
                    }
                }
                *left_n = c;
                mem::swap(right_n, left_n);
                // recalculating hight and count of the root
                *height_n = max(left_n.height(), right_n.height()) + 1;
                *count_n = left_n.count() + right_n.count() + 1;
            }
        }
    }
    
    fn is_balanced(&self)-> bool{
        match self {
            Self::Leaf => true,
            Self::Node { left, right, ..} => {
                let l_height = left.height();
                let r_height = right.height();
                return left.is_balanced() && right.is_balanced() && l_height + 1 >= r_height && r_height + 1 >= l_height;
            }
        }
    }

}

impl<T> Display for AvlTree<T>
where
    T: Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.simple_braces_string(0))
    }
}


impl<T> AvlTree<T>
where
T: Display
{
    fn simple_braces_string(&self, ebene: usize) -> String {
        let branch_symbols = "   ";
        match self {
            AvlTree::Leaf => branch_symbols.repeat(ebene) + "()\n",
            AvlTree::Node { elem, left, right, height, ..} => branch_symbols.repeat(ebene) + &format!("({elem}, {height}\n") + &left.simple_braces_string(ebene+1) + &right.simple_braces_string(ebene+1) + &branch_symbols.repeat(ebene) + ")\n"
        }
    }
    
}

impl<T> Default for AvlTree<T>{
    fn default() -> Self {
        Self::Leaf
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_degenerating_1_2_3_4_5_6_7_8_9_10(){
        let mut test_tree = AvlTree::new();
        test_tree.insert(1);
        assert!(test_tree.is_balanced());
        test_tree.insert(2);
        assert!(test_tree.is_balanced());
        test_tree.insert(3);
        assert!(test_tree.is_balanced());
        test_tree.insert(4);
        assert!(test_tree.is_balanced());
        test_tree.insert(5);
        assert!(test_tree.is_balanced());
        test_tree.insert(6);
        assert!(test_tree.is_balanced());
        test_tree.insert(7);
        assert!(test_tree.is_balanced());
        test_tree.insert(8);
        assert!(test_tree.is_balanced());
        test_tree.insert(9);
        assert!(test_tree.is_balanced());
        test_tree.insert(10);
        println!("{:?}", test_tree);
        assert!(test_tree.is_balanced());
    }
}