use std::cell::RefCell;
use std::rc::Rc;
use binary_lib::*;

// #[derive(Clone, Debug, PartialEq)]
type Tree = Rc<RefCell<TreeNode<u32>>>;
type AVLTree = Option<Tree>;

#[derive(Debug)]
struct TreeNode<T> {
    pub key: T,
    pub balance_factor: i32,
    pub parent: AVLTree,
    left: AVLTree,
    right: AVLTree,
}

impl <T: Ord> TreeNode<T>{
    fn new(data:T)-> Self {
        Self {
            key: data,
            balance_factor: 0,
            parent: None,
            left: None,
            right: None,
        }
    }
}

fn new_avl_tree(data: u32) -> Option<Tree> {
    Some(Rc::new(RefCell::new(TreeNode::new(data))))
}

fn find_key(avl_tree: Option<Tree>, data: u32) -> Option<Tree> {
    if let Some(node) = avl_tree.clone() {
        if data == node.borrow().key {
            return avl_tree;
        }
        if data < node.borrow().key {
            let left = &node.borrow().left;
            return find_key(left.clone(), data)
            
        } else {
            let right = &node.borrow().right;
            return find_key(right.clone(), data);
        }
    } else {
        None
    }
}

// Insert the newly added node into this function
fn rebalance_factor(avl_tree: &Option<Tree>) {
    if let Some(node) = avl_tree {
        let ancestor = &node.borrow().parent;
        if let Some(parent) = ancestor {
            let left_height = tree_height(&parent.borrow().left);
            let right_height = tree_height(&parent.borrow().right);
            let balance_factor = left_height - right_height;
            // Add calculations to change the balance factors here
            println!("the balance factor is {}", balance_factor.clone());
            parent.borrow_mut().balance_factor = balance_factor;
            rebalance_factor(ancestor);
        }
    }
}

fn insert_node(avl_tree: Option<Tree>, data: u32) -> Option<Tree> {
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

fn count_leaves(avl_tree: &Option<Tree>) -> i32 {
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

fn tree_height(avl_tree: &Option<Tree>) -> i32 {
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

fn in_order_traversal(avl_tree: &Option<Tree>, keys: &mut Vec<u32>) {
    if let Some(node) = avl_tree {
        let node_borrow = node.borrow();
        keys.push(node_borrow.key);
        if node_borrow.left.is_none() && node_borrow.right.is_none() {
            println!("{:?}", keys);
    
        }
        in_order_traversal(&node_borrow.left, keys);
        in_order_traversal(&node_borrow.right, keys);
        keys.pop();
    }
}

fn check_if_empty(avl_tree: &Option<Tree>) -> Result<(),()> {
    if avl_tree.is_some() {
        return Ok(())
    }
    else {
        return Err(())
    }
}

fn print_tree(avl_tree: &Option<Tree>, cur_level:usize){
    // dfs, with tabs for each level - 1
    if let Some(node) = avl_tree {
        for _ in 0..cur_level {
            let pad = "----";
            print!("{}",pad);
        }
        // let _ = std::iter::repeat(print!("-")).take(cur_level);
        let msg = format!(" {} ", node.borrow().key);
        println!("{:}", msg);
        
        print_tree(&node.borrow().left, cur_level+1);
        print_tree(&node.borrow().right, cur_level+1)
    }
}

// Insert the node to delete into this function
fn in_order_successor(avl_tree: Option<Tree>) -> Option<Tree> {
    if let Some(node) = avl_tree {
        let mut node_borrow = node.borrow().right.clone().unwrap();
        while let Some(left_child) = node_borrow.clone().borrow().left.clone() {
            node_borrow = left_child.clone()
        }
        return Some(node_borrow.clone())
    }
    None
}

fn remove_node(mut avl_tree: Option<Tree>, key: u32) -> Option<Tree> {
    if let Some(node) = find_key(avl_tree.clone(), key) {
        let node_key = node.borrow().key.clone();
        let parent = &node.borrow().parent.clone();
        //No children delete
        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            //Handle the case where deleted node is not the root
            if let Some(p_node) = parent {
                if node_key < p_node.borrow().key {
                    p_node.borrow_mut().left = None;
                }
                else if node_key > p_node.borrow().key {
                    p_node.borrow_mut().right = None;
                }
                return avl_tree
            }
            //Handle the case where the deleted node is the root
            else {
                return None
            }
        }
        //Right child delete
        if node.borrow().left.is_none() && !node.borrow().right.is_none() {
            let rep_node = node.borrow().right.clone();
            //Handle the case where the deleted node is not the root
            if let Some(p_node) = parent {
                if node_key < p_node.borrow().key {
                    p_node.borrow_mut().left = rep_node.clone();
                }
                else if node_key > p_node.borrow().key {
                    p_node.borrow_mut().right = rep_node.clone();
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
        if !node.borrow().left.is_none() && node.borrow().right.is_none() {
            let rep_node = node.borrow().left.clone();
            //Handle the case where the deleted node is not the root
            if let Some(p_node) = parent {
                if node_key < p_node.borrow().key {
                    p_node.borrow_mut().left = rep_node.clone();
                }
                else if node_key > p_node.borrow().key {
                    p_node.borrow_mut().right = rep_node.clone();
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
        if !node.borrow().left.is_none() && !node.borrow().right.is_none() {
            if let Some(rep_node) = in_order_successor(Some(node.clone())) {
                node.borrow_mut().key = rep_node.clone().borrow().key;
                let rep_node_parent = &rep_node.borrow().parent;
                if let Some(p_node) = rep_node_parent {
                    if rep_node.clone().borrow().key < p_node.clone().borrow().key {
                        p_node.clone().borrow_mut().left = None;
                    }
                    else if rep_node.clone().borrow().key > p_node.clone().borrow().key {
                        p_node.clone().borrow_mut().right = None;
                    }
                }
            }
        }
    }
    return avl_tree
}

/*
// This isnt complete yet
fn tree_layers(avl_tree: &AVLTree, counter: u32, string_vec: &mut Vec<String>) {
    if avl_tree.is_some() {
        let node_borrow = avl_tree.as_ref().unwrap().borrow();
        let key_string = node_borrow.key.to_string();
        if counter as usize <= tree_height(avl_tree) as usize - 1 {
            string_vec[counter as usize].push_str(&key_string);
        }
        // This may cause we seperate vec_strings to be returned
        tree_layers(&node_borrow.left, counter + 1, string_vec);
        tree_layers(&node_borrow.right, counter + 1, string_vec);
    }
    else {
        let key_string = "?".to_string();
        if counter as usize <= tree_height(avl_tree) as usize - 1 {
            string_vec[counter as usize].push_str(&key_string);
        }
    }
}
*/

// TO IMPLEMENT
// 1- DONE: Insert a node to the AVL tree.
// 2- DONE: Delete a node from the AVL tree.
// 3- DONE: Count the number of leaves in a tree.
// 4- DONE: Return the height of a tree.
// 5- DONE: Print In-order traversal of the tree.  -> Starting from root, then all lefts  
// 6- DONE: Check if the tree is empty.
// 7- DONE: Print the tree showing its structure. (Using println!(“{:#?}”,tree); is NOT
// sufficient).
// 8- Rebalance the tree for insert and delete.

fn main() {
    let mut avl_tree = new_avl_tree(25);
    avl_tree = insert_node(avl_tree, 12);
    avl_tree = insert_node(avl_tree, 37);
    avl_tree = insert_node(avl_tree, 6);
    avl_tree = insert_node(avl_tree, 15);
    let leaf_count = count_leaves(&avl_tree);
    let tree_height = tree_height(&avl_tree);
    let node_1 = find_key(avl_tree.clone(), 6);
    rebalance_factor(&node_1);
    let node_2 = find_key(avl_tree.clone(), 37);
    rebalance_factor(&node_2);
    println!("{}",leaf_count);
    println!("{}",tree_height);
    let mut keys: Vec<u32> = Vec::new();
    in_order_traversal(&avl_tree, &mut keys);
    print_tree(&avl_tree, 0);
    let result = check_if_empty(&avl_tree);
    let mut avl_tree = remove_node(avl_tree, 25);
    print_tree(&avl_tree, 0);
}
