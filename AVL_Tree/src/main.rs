use std::cell::RefCell;
use std::rc::Rc;
use binary_lib::*;

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    Right,
    Left,
    Root,
}

// #[derive(Clone, Debug, PartialEq)]
type Tree = Rc<RefCell<TreeNode<u32>>>;
type AVLTree = Option<Tree>;

#[derive(Debug)]
struct TreeNode<T> {
    pub balance_factor: i32,
    pub key: T,
    pub parent: AVLTree,
    left: AVLTree,
    right: AVLTree,
}

impl <T: Ord> TreeNode<T>{
    fn new(data:T)-> Self {
        Self {
            balance_factor: 0,
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

fn find_key(avl_tree: AVLTree, data: u32) -> AVLTree {
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
fn rebalance_factor(avl_tree: &AVLTree, data: u32) {
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
                let left_rotate = left_rotate(&avl_tree);
                rebalance_factor(avl_tree, data)
            }
            else if data < key {
                //Right-Left case
                let right_rotate = right_rotate(&right_node);
                let left_rotate = left_rotate(&avl_tree);
                rebalance_factor(avl_tree, data)
            }

        }
        else if balance_factor > 1 {
            //Either left left or left right case
            let left_node = node.clone().borrow().left.clone();
            let key = left_node.clone().unwrap().borrow().key.clone();
            if data < key {
                //Left-Left case
                let right_rotate = right_rotate(&avl_tree);
                rebalance_factor(avl_tree, data)
            }
            else if data > key {
                //Left-Right case
                let left_rotate = left_rotate(&left_node);
                let right_rotate = right_rotate(&avl_tree);
                rebalance_factor(avl_tree, data)
            }
        }
        //println!("the balance factor is {}", balance_factor.clone());
        else {
            node.borrow_mut().balance_factor = balance_factor;
            let ancestor = &node.clone().borrow().parent.clone();
            if let Some(parent) = ancestor {
                rebalance_factor(ancestor, data);
            }
        }
    }
}

fn insert_node(avl_tree: AVLTree, data: u32) -> AVLTree {
    if let Some(node) = avl_tree.clone() {
        if data < node.borrow().key {
            let left = node.borrow_mut().left.take();
            let new_left = insert_node(left, data);
            node.borrow_mut().left = new_left;
            if let Some(left_node) = &node.borrow().left.clone() {
                left_node.borrow_mut().parent = Some(node.clone());
                rebalance_factor(&avl_tree, data)
            }
        }
        else if data > node.borrow().key {
            let right = node.borrow_mut().right.take();
            let new_right = insert_node(right, data);
            node.borrow_mut().right = new_right;
            if let Some(right_node) = &node.borrow().right.clone() {
                right_node.borrow_mut().parent = Some(node.clone());
                rebalance_factor(&avl_tree, data)
            }
        }
        else {
            return None
        }
        return Some(node)
    }
    else {
        new_avl_tree(data.clone())
    }
}
/* 
fn insert(avl_tree: AVLTree, data: u32) -> AVLTree {
    let new_tree = insert_node(avl_tree, data);
    let current_node = &find_key(new_tree.clone(),data);
    let rebalance_tree = rebalance_factor(&node);
    if let Some(node) = current_node {

    }
}
*/
fn count_leaves(avl_tree: &AVLTree) -> u32 {
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

fn in_order_traversal(avl_tree: &AVLTree, keys: &mut Vec<u32>) {
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

fn print_tree(avl_tree: &AVLTree, cur_level:usize){
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
fn in_order_successor(avl_tree: AVLTree) -> AVLTree {
    if let Some(node) = avl_tree {
        let mut node_borrow = node.borrow().right.clone().unwrap();
        while let Some(left_child) = node_borrow.clone().borrow().left.clone() {
            node_borrow = left_child.clone()
        }
        return Some(node_borrow.clone())
    }
    None
}

fn remove_node(mut avl_tree: AVLTree, key: u32) -> AVLTree {
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
                let temp_key = rep_node.clone().borrow().key;
                let rep_node_parent = &rep_node.borrow().parent;
                if let Some(p_node) = rep_node_parent {
                    if rep_node.borrow().key < p_node.clone().borrow().key {
                        p_node.borrow_mut().left = None;
                        node.borrow_mut().key = temp_key;
                    }
                    else if rep_node.borrow().key > p_node.clone().borrow().key {
                        p_node.borrow_mut().right = None;
                        node.borrow_mut().key = temp_key;
                    }
                }
            }
        }
    }
    return avl_tree
}

//put the node to rotate into function
fn left_rotate(y: &AVLTree) -> AVLTree {
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
                println!("pointer");
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
fn right_rotate(y: &AVLTree) -> AVLTree {
    if let Some(y_node) = y {
        let z = y_node.clone().borrow_mut().parent.take();
        let x = y_node.clone().borrow_mut().left.take();
        y_node.borrow_mut().parent = x.clone();
        if let Some(x_node) = x {
            println!("yo {:?}", Rc::strong_count(&x_node));
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
    let mut avl_tree = new_avl_tree(1);
    avl_tree = insert_node(avl_tree, 2);
    avl_tree = insert_node(avl_tree, 3);
    avl_tree = insert_node(avl_tree, 4);
    avl_tree = insert_node(avl_tree, 5);
    avl_tree = insert_node(avl_tree, 6);
    avl_tree = insert_node(avl_tree, 7);
    avl_tree = insert_node(avl_tree, 8);
    avl_tree = insert_node(avl_tree, 9);
    let leaf_count = count_leaves(&avl_tree);
    let tree_height = tree_height(&avl_tree);
    let node_1 = find_key(avl_tree.clone(), 6);
    let node_2 = find_key(avl_tree.clone(), 37);
    println!("{}",leaf_count);
    println!("{}",tree_height);
    let mut keys: Vec<u32> = Vec::new();
    in_order_traversal(&avl_tree, &mut keys);
    print_tree(&avl_tree, 0);
    let result = check_if_empty(&avl_tree);
    let mut avl_tree = remove_node(avl_tree, 12);
    print_tree(&avl_tree, 0);
}
