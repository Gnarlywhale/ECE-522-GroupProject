use binary_lib::*;
use colored::*;
use std::cell::RefCell;
use std::env;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black,
}
#[derive(Clone, Debug, PartialEq)]
enum Direction {
    Right,
    Left,
}
type Tree = Rc<RefCell<TreeNode<u32>>>;
type RedBlackTree = Option<Tree>;

#[derive(Debug, PartialEq)]
struct TreeNode<T> {
    pub color: NodeColor,
    pub key: T,
    pub parent: RedBlackTree,
    left: RedBlackTree,
    right: RedBlackTree,
}

impl<T: Ord> TreeNode<T> {
    fn new(data: T) -> Self {
        Self {
            color: NodeColor::Red,
            key: data,
            parent: None,
            left: None,
            right: None,
        }
    }
}

// I think going with a struct-based implementation is a better way to go,
// But seems a bit weird since we have the RB tree defined as a foreign type, TO DISCUSS
// pub struct RBTree {
//     root: RedBlackTree
// }
// impl RBTree {
//     pub fn new() -> Self {
//         RBTree { root: None }
//     }

//     pub fn insert_node(&mut self, data:T){

//     }
// }

fn new_rb_tree(data: u32) -> RedBlackTree {
    let mut node = TreeNode::new(data);
    node.color = NodeColor::Black;
    Some(Rc::new(RefCell::new(node)))
}
fn new_rb_node(data: u32) -> RedBlackTree {
    Some(Rc::new(RefCell::new(TreeNode::new(data))))
}

fn find_key(rb_tree: RedBlackTree, data: u32) -> RedBlackTree {
    if let Some(node) = rb_tree.clone() {
        if data == node.borrow().key {
            return rb_tree;
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
fn in_order_successor(rb_tree: RedBlackTree) -> RedBlackTree {
    if let Some(node) = rb_tree {
        //
        let mut curr_node = node.borrow().right.clone().unwrap();
        while let Some(l_node) = curr_node.clone().borrow().left.clone() {
            curr_node = l_node.clone();
        }
        // Found last successor
        return Some(curr_node.clone());
        // print_tree(&Some(curr_node.clone()), 0)
    }
    None
}

// TODO double check edge cases! i.e. 1 node tree
fn remove_node(mut rb_tree: RedBlackTree, data: u32) -> RedBlackTree {
    if let Some(node) = find_key(rb_tree.clone(), data) {
        // Handle terminal node case
        let node_key = node.borrow().key.clone();
        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            let parent = &node.borrow().parent;
            if let Some(p_node) = parent {
                if node_key < p_node.borrow().key {
                    p_node.borrow_mut().left = None;
                } else {
                    p_node.borrow_mut().right = None;
                }
                return rb_tree;
            } else {
                return None;
            }
        } else if node.borrow().left.is_none() && node.borrow().right.is_some() {
            // One right child
            let rep_node = node.borrow().right.clone();
            let parent = node.borrow().parent.clone();
            if let Some(p_node) = parent {
                if node_key < p_node.borrow().key {
                    p_node.borrow_mut().left = rep_node.clone();
                } else {
                    p_node.borrow_mut().right = rep_node.clone();
                }
            } else {
                node.borrow_mut().key = rep_node.unwrap().borrow().key;
                node.borrow_mut().parent = None;
                node.borrow_mut().left = None;
                node.borrow_mut().right = None
            }
        } else if node.borrow().left.is_some() && node.borrow().right.is_none() {
            // One left child
            let rep_node = node.borrow().left.clone();
            let parent = node.borrow().parent.clone();
            if let Some(p_node) = parent {
                if node_key < p_node.borrow().key {
                    p_node.borrow_mut().left = rep_node.clone();
                } else {
                    p_node.borrow_mut().right = rep_node.clone();
                }
            } else {
                node.borrow_mut().key = rep_node.unwrap().borrow().key;
                node.borrow_mut().parent = None;
                node.borrow_mut().left = None;
                node.borrow_mut().right = None
            }
        } else {
            if let Some(rep_node) = in_order_successor(Some(node.clone())) {
                // Set the parent key to match the rep_node key
                node.borrow_mut().key = rep_node.clone().borrow().key;
                // Remove the reference from the replacement node's parent to the replacement node
                let parent = &rep_node.borrow().parent;
                if let Some(p_node) = parent {
                    if rep_node.clone().borrow().key < p_node.clone().borrow().key {
                        p_node.clone().borrow_mut().left = None;
                    } else {
                        p_node.clone().borrow_mut().right = None;
                    }
                }
            }
        }
    }
    return rb_tree;
}
fn insert_node(rb_tree: RedBlackTree, data: u32) -> RedBlackTree {
    if let Some(node) = rb_tree {
        if data == node.borrow().key {
            return None;
        }
        if data < node.borrow().key {
            let left = node.borrow_mut().left.take();
            let new_left = insert_node(left, data);
            node.borrow_mut().left = new_left;
            if let Some(left_node) = &node.borrow().left {
                left_node.borrow_mut().parent = Some(node.clone());
            }
        } else {
            let right = node.borrow_mut().right.take();
            let new_right = insert_node(right, data);
            node.borrow_mut().right = new_right;
            if let Some(right_node) = &node.borrow().right {
                right_node.borrow_mut().parent = Some(node.clone());
            }
        }
        Some(node)
    } else {
        new_rb_node(data)
    }
}

fn insert(rb_tree: RedBlackTree, data: u32) -> RedBlackTree {
    let new_tree = insert_node(rb_tree, data);
    let node = &find_key(new_tree.clone(), data);
    let rotates = insert_balance(node);
    println!("{:#?}", rotates);
    for (direction, key) in rotates {
        match direction {
            Direction::Left => {
                let r_node = &find_key(new_tree.clone(), key);
                let root = left_rotate(r_node);
                if root.is_some() {
                    return root;
                }
            }
            Direction::Right => {
                let r_node = &find_key(new_tree.clone(), key);
                let root = right_rotate(r_node);
                if root.is_some() {
                    return root;
                }
            }
        }
    }
    return new_tree;
}

fn insert_balance(x: &RedBlackTree) -> Vec<(Direction, u32)> {
    if let Some(x_node) = x {
        println!("pointer: start balance");
        let mut x_dir = Direction::Left;
        let p = x_node.clone().borrow().parent.clone();
        if p.is_none() {
            x_node.clone().borrow_mut().color = NodeColor::Black;
        } else if let Some(p_node) = p {
            if let Some(right) = &p_node.clone().borrow().right {
                if right.clone().borrow().key == x_node.clone().borrow().key {
                    x_dir = Direction::Right;
                }
            }
            if p_node.clone().borrow().color == NodeColor::Red {
                let gp = p_node.clone().borrow().parent.clone();
                if let Some(gp_node) = gp {
                    let mut p_dir = Direction::Right;
                    if let Some(left) = gp_node.clone().borrow().left.clone() {
                        if left.borrow().key == p_node.clone().borrow().key {
                            p_dir = Direction::Left;
                        }
                    }
                    match p_dir {
                        Direction::Left => {
                            let u = gp_node.clone().borrow().right.clone();
                            if let Some(ref u_node) = u {
                                if u_node.borrow().color == NodeColor::Red {
                                    p_node.clone().borrow_mut().color = NodeColor::Black;
                                    u_node.borrow_mut().color = NodeColor::Black;
                                    gp_node.borrow_mut().color = NodeColor::Red;
                                    println!("pointer: recursive");
                                    return insert_balance(&p_node.clone().borrow().parent);
                                } else {
                                    match x_dir {
                                        Direction::Right => {
                                            x_node.clone().borrow_mut().color = NodeColor::Black;
                                            gp_node.clone().borrow_mut().color = NodeColor::Red;
                                            let mut rotate = Vec::new();
                                            rotate.push((Direction::Left, p_node.borrow().key));
                                            rotate.push((Direction::Right, gp_node.borrow().key));
                                            return rotate;
                                        }
                                        Direction::Left => {
                                            gp_node.borrow_mut().color = NodeColor::Red;
                                            p_node.clone().borrow_mut().color = NodeColor::Black;
                                            let mut rotate = Vec::new();
                                            rotate.push((Direction::Right, gp_node.borrow().key));
                                            return rotate;
                                        }
                                    }
                                }
                            } else {
                                match x_dir {
                                    Direction::Right => {
                                        x_node.clone().borrow_mut().color = NodeColor::Black;
                                        gp_node.clone().borrow_mut().color = NodeColor::Red;
                                        let mut rotate = Vec::new();
                                        rotate.push((Direction::Left, p_node.borrow().key));
                                        rotate.push((Direction::Right, gp_node.borrow().key));
                                        return rotate;
                                    }
                                    Direction::Left => {
                                        gp_node.borrow_mut().color = NodeColor::Red;
                                        p_node.clone().borrow_mut().color = NodeColor::Black;
                                        let mut rotate = Vec::new();
                                        rotate.push((Direction::Right, gp_node.borrow().key));
                                        return rotate;
                                    }
                                }
                            }
                        }
                        Direction::Right => {
                            let u = gp_node.clone().borrow().left.clone();
                            if let Some(ref u_node) = u {
                                if u_node.borrow().color == NodeColor::Red {
                                    p_node.borrow_mut().color = NodeColor::Black;
                                    u_node.borrow_mut().color = NodeColor::Black;
                                    gp_node.borrow_mut().color = NodeColor::Red;
                                    println!("pointer: recursive");
                                    return insert_balance(&p_node.clone().borrow().parent);
                                } else {
                                    match x_dir {
                                        Direction::Right => {
                                            gp_node.borrow_mut().color = NodeColor::Red;
                                            p_node.clone().borrow_mut().color = NodeColor::Black;
                                            let mut rotate = Vec::new();
                                            rotate.push((Direction::Left, gp_node.borrow().key));
                                            return rotate;
                                        }
                                        Direction::Left => {
                                            x_node.borrow_mut().color = NodeColor::Black;
                                            gp_node.borrow_mut().color = NodeColor::Red;
                                            let mut rotate = Vec::new();
                                            rotate.push((Direction::Right, p_node.borrow().key));
                                            rotate.push((Direction::Left, gp_node.borrow().key));
                                            return rotate;
                                        }
                                    }
                                }
                            } else {
                                match x_dir {
                                    Direction::Right => {
                                        gp_node.borrow_mut().color = NodeColor::Red;
                                        p_node.clone().borrow_mut().color = NodeColor::Black;
                                        let mut rotate = Vec::new();
                                        rotate.push((Direction::Left, gp_node.borrow().key));
                                        return rotate;
                                    }
                                    Direction::Left => {
                                        x_node.borrow_mut().color = NodeColor::Black;
                                        gp_node.borrow_mut().color = NodeColor::Red;
                                        let mut rotate = Vec::new();
                                        rotate.push((Direction::Right, p_node.borrow().key));
                                        rotate.push((Direction::Left, gp_node.borrow().key));
                                        return rotate;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Vec::new()
}

fn get_child(opt_node: RedBlackTree, direction: Direction) -> RedBlackTree {
    if let Some(node) = opt_node {
        match direction {
            Direction::Right => return node.borrow().right.clone(),
            Direction::Left => return node.borrow().left.clone(),
        }
    }
    None
}

fn left_rotate(y: &RedBlackTree) -> RedBlackTree {
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

fn right_rotate(y: &RedBlackTree) -> RedBlackTree {
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

// TO IMPLEMENT
// 1- Insert a node to the red-black tree.
// 2- Delete a node from the red-black tree.
// 3- Count the number of leaves in a tree.
// 4- Return the height of a tree.
// 5- Print In-order traversal of the tree.  -> Starting from root, then all lefts
// 6- Check if the tree is empty.
// 7- Print the tree showing its colors and structure. (Using println!(“{:#?}”,tree); is NOT
// sufficient)
fn main() {
    // uncomment for more verbose error messages
    // env::set_var("RUST_BACKTRACE", "1");
    let mut rb_tree = new_rb_tree(1);
    rb_tree = insert(rb_tree, 2);
    rb_tree = insert(rb_tree, 3);
    rb_tree = insert(rb_tree, 4);
    rb_tree = insert(rb_tree, 5);
    rb_tree = insert(rb_tree, 6);
    rb_tree = insert(rb_tree, 7);
    rb_tree = insert(rb_tree, 8);
    rb_tree = insert(rb_tree, 9);
    // rb_tree = insert(rb_tree, 10);
    // rb_tree = insert(rb_tree, 11);
    // rb_tree = insert(rb_tree, 12);
    // rb_tree = insert(rb_tree, 13);
    print_tree(&rb_tree, 0);
    // let print_node = find_key(rb_tree.clone(), 20);
    // print_tree(&print_node,0);

    // Edge case delete testing
    //     let mut smol_tree = new_rb_tree(42);
    //     println!("Delete root for left node");
    //     smol_tree = insert_node(smol_tree, 40);
    //     print_tree(&smol_tree, 0);
    //     smol_tree = remove_node(smol_tree, 42);
    //     print_tree(&smol_tree, 0);
    //     println!("Delete root for right node");
    // smol_tree = insert_node(smol_tree, 50);
    // print_tree(&smol_tree, 0);
    // smol_tree = remove_node(smol_tree, 40);
    //     print_tree(&smol_tree, 0);
    //     println!("Delete root for no node");
    // smol_tree = remove_node(smol_tree, 50);
    // print_tree(&smol_tree, 0);
    // println!("Done edge case testing");
    // let mut rb_tree = new_rb_tree(42);
    // rb_tree = insert_node(rb_tree, 500);
    // rb_tree = insert_node(rb_tree, 30);
    // rb_tree = insert_node(rb_tree, 45);
    // rb_tree = insert_node(rb_tree, 55);
    // rb_tree = insert_node(rb_tree, 20);
    // rb_tree = insert_node(rb_tree, 35);
    // rb_tree = insert_node(rb_tree, 33);
    // rb_tree = insert_node(rb_tree, 31);
    // rb_tree = insert_node(rb_tree, 34);
    // rb_tree = insert_node(rb_tree, 36);
    // rb_tree = insert_node(rb_tree, 3);
    // rb_tree = insert_node(rb_tree, 25);
    // println!("Sample Tree:");
    // print_tree(&rb_tree.clone(), 0);
    // rb_tree = remove_node(rb_tree.clone(), 30);
    // println!("Should now show 31 in place of 30, and have 30 removed (in-order successor)");
    // print_tree(&rb_tree.clone(), 0);
    // println!("Deleting terminal node: 25");
    // remove_node(rb_tree.clone(), 25);
    // print_tree(&rb_tree.clone(), 0);
    // println!("Deleting single child node: 500");
    // remove_node(rb_tree.clone(), 500);
    // print_tree(&rb_tree.clone(), 0);
    // rb_tree = left_rotate(rb_tree).unwrap();
}

// impl <T: Ord> TreeNode<T>{
// fn new(data:T)-> Self {
// fn new(data:T, node_parent:Option<TreeNode<u32>>)-> Self {
// if let Some(np) = node_parent{
//     Self {
//         color: NodeColor::Red,
//         key: data,
//         parent: Some(Rc::new(RefCell::new(node_parent.unwrap()))),
//         left: None,
//         right: None,
//     }
// } else {
//         Self {
//             color: NodeColor::Red,
//             key: data,
//             parent: None,
//             left: None,
//             right: None,
//         }
// }

// }
// }

//
// Swap commented code to allow null prints to help clarify which children are left and right nodes
// Could probably implement both versions as an added feature
fn print_tree(rb_tree: &RedBlackTree, cur_level: usize) {
    // for i in 0..cur_level {

    //     let pad: &str;
    //     if i == cur_level-1{
    //         pad = " |→";
    //     } else {
    //         pad = " |  ";
    //     }
    //     print!("{}",pad.on_white());
    // }

    // dfs, with tabs for each level - 1
    if let Some(node) = rb_tree {
        for i in 0..cur_level {
            let pad: &str;
            if i == cur_level - 1 {
                pad = " |→";
            } else {
                pad = " |  ";
            }
            print!("{}", pad.on_white());
        }
        // let _ = std::iter::repeat(print!("-")).take(cur_level);
        let msg = format!(" {} ", node.borrow().key);
        if node.borrow().color == NodeColor::Black {
            println!("{:}", msg.black().on_white());
        } else {
            println!("{:}", msg.red().on_white());
        }

        print_tree(&node.borrow().left, cur_level + 1);
        print_tree(&node.borrow().right, cur_level + 1)
    } else {
        // println!();
    }
}
