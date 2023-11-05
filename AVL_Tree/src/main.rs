use std::cell::RefCell;
use std::rc::Rc;
use binary_lib::*;

// #[derive(Clone, Debug, PartialEq)]
type Tree = Rc<RefCell<TreeNode<u32>>>;
type AVLTree = Option<Tree>;
#[derive(Debug)]
struct TreeNode<T> {
    pub height: i32,
    pub key: T,
    pub parent: AVLTree,
    left: AVLTree,
    right: AVLTree,
}

impl <T: Ord> TreeNode<T>{
    fn new(data:T)-> Self {
        Self {
            height: 0,
            key: data,
            parent: None,
            left: None,
            right: None,
        }
    }
    
}

fn main() {
    test_function()
}
