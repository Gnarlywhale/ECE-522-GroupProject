use binary_lib::*;
use std::cell::RefCell;
use std::rc::Rc;
use colored::*;

// #[derive(Clone, Debug, PartialEq)]
pub type Tree = Rc<RefCell<TreeNode<u32>>>;
pub type AVLTree = Option<Tree>;

#[derive(Debug)]
pub struct TreeNode<T> {
    pub balance_factor: i32,
    pub key: T,
    pub parent: AVLTree,
    pub left: AVLTree,
    pub right: AVLTree,
}

impl<T: Ord> TreeNode<T> {
    pub fn new(data: T) -> Self {
        Self {
            balance_factor: 0,
            key: data,
            parent: None,
            left: None,
            right: None,
        }
    }
}

pub fn new_avl_tree(data: u32) -> AVLTree {
    Some(Rc::new(RefCell::new(TreeNode::new(data))))
}

pub fn find_key(avl_tree: AVLTree, data: u32) -> AVLTree {
    if let Some(node) = avl_tree.clone() {
        if data == node.borrow().key {
            return avl_tree;
        }
        if data < node.borrow().key {
            let left = &node.borrow().left;
            return find_key(left.clone(), data);
        } else {
            let right = &node.borrow().right;
            return find_key(right.clone(), data);
        }
    } else {
        None
    }
}

// Insert the newly added node into this function
pub fn rebalance_factor(avl_tree: &AVLTree, data: u32) -> AVLTree {
    if let Some(node) = avl_tree.clone() {
        let left_height = tree_height(&node.clone().borrow().left.clone());
        let right_height = tree_height(&node.clone().borrow().right.clone());
        let balance_factor = left_height - right_height;
        if balance_factor < -1 {
            //Either right right or right left case
            let right_node = node.clone().borrow().right.clone();
            let key = right_node.clone().unwrap().borrow().key.clone();
            if data > key {
                //Right-Right case
                left_rotate(&avl_tree);
                return rebalance_factor(avl_tree, data);
            } else if data < key {
                //Right-Left case
                right_rotate(&right_node);
                left_rotate(&avl_tree);
                return rebalance_factor(avl_tree, data);
            }
        } else if balance_factor > 1 {
            //Either left left or left right case
            let left_node = node.clone().borrow().left.clone();
            let key = left_node.clone().unwrap().borrow().key.clone();
            if data < key {
                //Left-Left case
                right_rotate(&avl_tree);
                return rebalance_factor(avl_tree, data);
            } else if data > key {
                //Left-Right case
                left_rotate(&left_node);
                right_rotate(&avl_tree);
                return rebalance_factor(avl_tree, data);
            }
        }
        //println!("the balance factor is {}", balance_factor.clone());
        else {
            node.borrow_mut().balance_factor = balance_factor;
            let ancestor = &node.clone().borrow().parent.clone();
            if ancestor.is_some() {
                return rebalance_factor(ancestor, data);
            } else {
                return avl_tree.clone();
            }
        }
    }
    None
}

pub fn insert(avl_tree: &AVLTree, data: u32) -> AVLTree {
    let new_tree = insert_node(avl_tree.clone(), data);
    let new_node = find_key(new_tree.clone(), data);
    let balance_tree = rebalance_factor(&new_node, data);
    if balance_tree.is_some(){
        return balance_tree;
    }
    return new_tree;
}

pub fn insert_node(avl_tree: AVLTree, data: u32) -> AVLTree {
    if let Some(node) = avl_tree.clone() {
        if data < node.borrow().key {
            let left = node.borrow_mut().left.take();
            let new_left = insert_node(left, data);
            node.borrow_mut().left = new_left;
            if let Some(left_node) = &node.borrow().left.clone() {
                left_node.borrow_mut().parent = Some(node.clone());
                //rebalance_factor(&avl_tree, data)
            }
        } else if data > node.borrow().key {
            let right = node.borrow_mut().right.take();
            let new_right = insert_node(right, data);
            node.borrow_mut().right = new_right;
            if let Some(right_node) = &node.borrow().right.clone() {
                right_node.borrow_mut().parent = Some(node.clone());
                //rebalance_factor(&avl_tree, data)
            }
        } else {
            return None;
        }
        return Some(node);
    } else {
        new_avl_tree(data.clone())
    }
}
/*
pub fn insert(avl_tree: AVLTree, data: u32) -> AVLTree {
    let new_tree = insert_node(avl_tree, data);
    let current_node = &find_key(new_tree.clone(),data);
    let rebalance_tree = rebalance_factor(&node);
    if let Some(node) = current_node {

    }
}
*/
pub fn count_leaves(avl_tree: &AVLTree) -> u32 {
    if let Some(node) = avl_tree {
        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            return 1;
        } else {
            return count_leaves(&node.borrow().left) + count_leaves(&node.borrow().right);
        }
    } else {
        return 0;
    }
}

pub fn count_nodes(avl_tree: &AVLTree) -> u32 {
    if let Some(node) = avl_tree {
        if node.borrow().left.is_some() && node.borrow().right.is_none() {
            return 1 + count_nodes(&node.borrow().left)
        }
        else if node.borrow().right.is_some() && node.borrow().left.is_none() {
            return 1 + count_nodes(&node.borrow().right)
        }
        else if node.borrow().left.is_some() && node.borrow().right.is_some() {
            return 1 + count_nodes(&node.borrow().left) + count_nodes(&node.borrow().right)
        }
        else {
            return 1;
        }
    }
    else {
        return 0;
    }
}

pub fn tree_height(avl_tree: &AVLTree) -> i32 {
    if let Some(node) = avl_tree {
        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            return 1;
        } else {
            let left_height = tree_height(&node.borrow().left);
            let right_height = tree_height(&node.borrow().right);
            return 1 + i32::max(left_height, right_height);
        }
    } else {
        return 0;
    }
}

//Clean-up this later to print better looking branches.
pub fn in_order_traversal(avl_tree: &AVLTree, keys: &mut Vec<u32>) {
    if let Some(node) = avl_tree {
        let node_borrow = node.borrow();
        keys.push(node_borrow.key);
        in_order_traversal(&node_borrow.left, keys);
        in_order_traversal(&node_borrow.right, keys);
    }
}

pub fn check_if_empty(avl_tree: &Option<Tree>) -> Result<(), ()> {
    if avl_tree.is_some() {
        return Ok(());
    } else {
        return Err(());
    }
}
pub fn print_tree(avl_tree: &AVLTree, cur_level: usize) {
    for i in 0..cur_level {

        let pad: &str;
        if i == cur_level-1{
            pad = " |→";
        } else {
            pad = " |  ";
        }
        print!("{}",pad.black().on_white());
    }

    // dfs, with tabs for each level - 1
    if let Some(node) = avl_tree {
        // for i in 0..cur_level {
        //     let pad: &str;
        //     if i == cur_level - 1 {
        //         pad = " |→";
        //     } else {
        //         pad = " |  ";
        //     }
        //     print!("{}", pad.on_white());
        // }
        // let _ = std::iter::repeat(print!("-")).take(cur_level);
        let msg = format!(" {} ", node.borrow().key);
        println!("{:}", msg.black().on_white());

        print_tree(&node.borrow().left, cur_level + 1);
        print_tree(&node.borrow().right, cur_level + 1)
    } else {
        println!();
    }
}
/*
pub fn print_tree(avl_tree: &AVLTree, cur_level: usize) {
    // dfs, with tabs for each level - 1
    if let Some(node) = avl_tree {
        for _ in 0..cur_level {
            let pad = "----";
            print!("{}", pad);
        }
        // let _ = std::iter::repeat(print!("-")).take(cur_level);
        let msg = format!(" {} ", node.borrow().key);
        println!("{:}", msg);

        print_tree(&node.borrow().left, cur_level + 1);
        print_tree(&node.borrow().right, cur_level + 1)
    }
}
*/
// Insert the node to of importance into this function
pub fn in_order_successor(avl_tree: AVLTree) -> AVLTree {
    if let Some(node) = avl_tree {
        let mut node_borrow = node.borrow().right.clone().unwrap();
        while let Some(left_child) = node_borrow.clone().borrow().left.clone() {
            node_borrow = left_child.clone();
        }
        return Some(node_borrow.clone());
    }
    None
}

pub fn delete(mut avl_tree: AVLTree, key: u32) -> AVLTree {
    let (new_tree, deleted_node_parent) = remove_node(avl_tree, key.clone());
    let balance_tree = rebalance_factor(&deleted_node_parent, key);
    if balance_tree.is_some() {
        return balance_tree
    }
    else {
         return new_tree
    }
}

//insert full tree
pub fn remove_node(mut avl_tree: AVLTree, key: u32) -> (AVLTree, AVLTree) {
    let mut replacement_node_parent: Option<Rc<RefCell<TreeNode<u32>>>> = None;
    if let Some(node) = find_key(avl_tree.clone(), key) {
        let node_key = node.clone().borrow().key.clone();
        let parent = &node.borrow().parent.clone();
        replacement_node_parent = parent.clone();
        //No children delete
        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            //Handle the case where deleted node is not the root
            if let Some(p_node) = parent {
                if node_key < p_node.borrow().key {
                    p_node.borrow_mut().left = None;
                } else if node_key > p_node.borrow().key {
                    p_node.borrow_mut().right = None;
                }
                return (avl_tree, replacement_node_parent);
            }
            //Handle the case where the deleted node is the root
            else {
                return (None, replacement_node_parent);
            }
        }
        //Right child delete
        if node.borrow().left.is_none() && node.borrow().right.is_some() {
            let rep_node = node.borrow().right.clone();
            //Handle the case where the deleted node is not the root
            if let Some(p_node) = parent {
                if node_key < p_node.borrow().key {
                    p_node.borrow_mut().left = rep_node.clone();
                    rep_node.clone().unwrap().borrow_mut().parent = parent.clone();
                } else if node_key > p_node.borrow().key {
                    p_node.borrow_mut().right = rep_node.clone();
                    rep_node.unwrap().clone().borrow_mut().parent = parent.clone();
                }
            }
            //Handle the case where the deleted node is the root
            else {
                node.borrow_mut().key = rep_node.unwrap().borrow().key;
                node.borrow_mut().parent = None;
                node.borrow_mut().left = None;
                node.borrow_mut().right = None;
            }
        }
        //Left child delete
        if node.borrow().left.is_some() && node.borrow().right.is_none() {
            let rep_node = node.borrow().left.clone();
            //Handle the case where the deleted node is not the root
            if let Some(p_node) = parent {
                if node_key < p_node.borrow().key {
                    p_node.borrow_mut().left = rep_node.clone();
                    rep_node.clone().unwrap().borrow_mut().parent = parent.clone()
                } else if node_key > p_node.borrow().key {
                    p_node.borrow_mut().right = rep_node.clone();
                    rep_node.clone().unwrap().borrow_mut().parent = parent.clone()
                }
            }
            //Handle the case where the deleted node is the root
            else {
                node.borrow_mut().key = rep_node.unwrap().borrow().key;
                node.borrow_mut().parent = None;
                node.borrow_mut().left = None;
                node.borrow_mut().right = None;
            }
        }
        //Two children delete
        if node.borrow().left.is_some() && node.borrow().right.is_some() {
            if let Some(rep_node) = in_order_successor(Some(node.clone())) {
                let temp_key = rep_node.clone().borrow().key;
                let rep_node_parent = &rep_node.borrow().parent.clone();
                let rep_node_left_child = rep_node.clone().borrow().left.clone();
                let rep_node_right_child = rep_node.clone().borrow().right.clone();
                if let Some(p_node) = rep_node_parent {
                    if rep_node_left_child.is_none() && rep_node_right_child.is_none() {
                        if rep_node.borrow().key < p_node.clone().borrow().key {
                            p_node.borrow_mut().left = None;
                            node.borrow_mut().key = temp_key;
                        } else if rep_node.borrow().key > p_node.clone().borrow().key {
                            p_node.borrow_mut().right = None;
                            node.borrow_mut().key = temp_key;
                        }
                    }
                    else if rep_node_right_child.is_some() {
                        if rep_node.borrow().key > p_node.clone().borrow().key {
                            rep_node.borrow_mut().key = rep_node_right_child.unwrap().borrow().key.clone();
                            rep_node.borrow_mut().right = None;
                            node.borrow_mut().key = temp_key;
                        }
                    }
                }
            }
        }
    }
    return (avl_tree, replacement_node_parent);
}

//put the node to rotate into function
pub fn left_rotate(y: &AVLTree) -> AVLTree {
    if let Some(y_node) = y.clone() {
        let temp = y_node.clone();
        let z = temp.borrow().parent.clone();
        let x = temp.borrow().right.clone();
        y_node.borrow_mut().parent = x.clone();
        if let Some(x_node) = x {
            let child = x_node.clone().borrow().left.clone();
            x_node.clone().borrow_mut().parent = z.clone();
            y_node.borrow_mut().right = child.clone();
            x_node.borrow_mut().left = Some(y_node.clone());
            if let Some(child_node) = child {
                child_node.borrow_mut().parent = Some(y_node.clone());
            }
            if let Some(z_node) = z {
                let mut left = false;
                if let Some(left_node) = &z_node.clone().borrow().left {
                    if left_node.clone().borrow().key == y_node.clone().borrow().key {
                        left = true;
                    }
                }
                if left {
                    z_node.borrow_mut().left = Some(x_node);
                } else {
                    z_node.borrow_mut().right = Some(x_node);
                }
            } else {
                return Some(x_node);
            }
        }
    }
    None
}

//put node to rotate into function
pub fn right_rotate(y: &AVLTree) -> AVLTree {
    if let Some(y_node) = y {
        let z = y_node.clone().borrow_mut().parent.take();
        let x = y_node.clone().borrow_mut().left.take();
        y_node.borrow_mut().parent = x.clone();
        if let Some(x_node) = x {
            let child = x_node.borrow_mut().right.take();
            x_node.borrow_mut().parent = z.clone();
            y_node.borrow_mut().left = child.clone();
            x_node.borrow_mut().right = Some(y_node.clone());
            if let Some(child_node) = child {
                child_node.borrow_mut().parent = Some(y_node.clone());
            }
            if let Some(z_node) = z {
                let mut left = false;
                if let Some(left_node) = &z_node.clone().borrow().left {
                    if left_node.clone().borrow().key == y_node.clone().borrow().key {
                        left = true;
                    }
                }
                if left {
                    z_node.borrow_mut().left = Some(x_node);
                } else {
                    z_node.borrow_mut().right = Some(x_node);
                }
            } else {
                return Some(x_node);
            }
        }
    }
    None
}