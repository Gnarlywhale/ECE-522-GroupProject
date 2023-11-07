use std::cell::RefCell;
use std::rc::Rc;
use binary_lib::*;

// #[derive(Clone, Debug, PartialEq)]
type Tree = Rc<RefCell<TreeNode<u32>>>;
type AVLTree = Option<Tree>;

#[derive(Debug)]
struct TreeNode<T> {
    pub key: T,
    pub parent: AVLTree,
    left: AVLTree,
    right: AVLTree,
}

impl <T: Ord> TreeNode<T>{
    fn new(data:T)-> Self {
        Self {
            key: data,
            parent: None,
            left: None,
            right: None,
        }
    }
}

fn new_avl_tree(data: u32) -> AVLTree {
    Some(Rc::new(RefCell::new(TreeNode::new(data))))
}

fn insert_node(avl_tree: AVLTree, data: u32) -> AVLTree {
    if let Some(node) = avl_tree {
        if data < node.borrow().key {
            let left = node.borrow_mut().left.take();
            let new_left = insert_node(left, data);
            node.borrow_mut().left = new_left;
            if let Some(left_node) = &node.borrow().left {
                left_node.borrow_mut().parent = Some(node.clone());
            }
        }
        else if data > node.borrow().key {
            let right = node.borrow_mut().right.take();
            let new_right = insert_node(right, data);
            node.borrow_mut().right = new_right;
            if let Some(right_node) = &node.borrow().right {
                right_node.borrow_mut().parent = Some(node.clone());
            }
        }
        else {
            return None
        }
        Some(node)
    }
    else {
        new_avl_tree(data)
    }
}

fn count_leaves(avl_tree: &AVLTree) -> i32 {
    if let Some(node) = avl_tree {
        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            return 1
        }
        else {
            return count_leaves(&node.borrow().left) + count_leaves(&node.borrow().right)
        }
    }
    else {
        return 0
    }
}

fn tree_height(avl_tree: &AVLTree) -> i32 {
    if let Some(node) = avl_tree {
        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            return 1
        }
        else {
            let left_height = tree_height(&node.borrow().left);
            let right_height = tree_height(&node.borrow().right);
            return 1 + i32::max(left_height, right_height)
        }
    }
    else {
        return -1
    }
}


// TO IMPLEMENT
// 1- DONE: Insert a node to the AVL tree.
// 2- Delete a node from the AVL tree.
// 3- DONE: Count the number of leaves in a tree.
// 4- DONE: Return the height of a tree.
// 5- Print In-order traversal of the tree.  -> Starting from root, then all lefts  
// 6- Check if the tree is empty.
// 7- Print the tree showing its structure. (Using println!(“{:#?}”,tree); is NOT
// sufficient)
fn main() {
    let mut avl_tree = new_avl_tree(25);
    avl_tree = insert_node(avl_tree, 12);
    avl_tree = insert_node(avl_tree, 37);
    avl_tree = insert_node(avl_tree, 6);
    avl_tree = insert_node(avl_tree, 15);
    let leaf_count = count_leaves(&avl_tree);
    let tree_height = tree_height(&avl_tree);
    println!("{}",leaf_count);
    println!("{}",tree_height);
}
