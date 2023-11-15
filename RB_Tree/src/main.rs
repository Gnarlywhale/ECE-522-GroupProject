use binary_lib::*;
use std::cell::RefCell;
use std::rc::Rc;
use colored::*;

#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black,
}

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
    Some(Rc::new(RefCell::new(TreeNode::new(data))))
}



fn find_key(rb_tree: RedBlackTree, data: u32) -> RedBlackTree {
    if let Some(node) = rb_tree.clone() {
        if data == node.borrow().key {
            return rb_tree;
        }
        if data < node.borrow().key {
            let left = &node.borrow().left;
            return find_key(left.clone(), data)
            
        } else {
            let right = node.borrow_mut().right.take();
            return find_key(right, key);
        }
    } else {
        None
    }
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
        new_rb_tree(data)
    }
}

fn insert(rb_tree: RedBlackTree, data: u32)->RedBlackTree{
    let new_tree = insert_node(rb_tree, data);
    insert_balance(&find_key(&new_tree, data));
    return new_tree;
}

fn insert_balance(x: &RedBlackTree) {
    if let Some(ref x_node) = x {
        let parent = &x_node.borrow_mut().parent;
        let mut x_dir = Direction::Left;
        if x_node.borrow().parent.is_none() {
            x_node.borrow_mut().color = NodeColor::Black;
        } else if let Some(ref p_node) = x_node.borrow_mut().parent {
            let grandparent = &p_node.borrow_mut().parent;
            if p_node.borrow().right == *x {
                x_dir = Direction::Right;
            }
            if p_node.borrow().color == NodeColor::Red {
                if let Some(ref gp_node) = p_node.borrow_mut().parent {
                    if gp_node.borrow().right == x_node.borrow().parent {
                        if let Some(ref u_node) = p_node.borrow_mut().right {
                            if u_node.borrow().color == NodeColor::Red {
                                p_node.borrow_mut().color = NodeColor::Black;
                                u_node.borrow_mut().color = NodeColor::Black;
                                gp_node.borrow_mut().color = NodeColor::Red;
                                insert_balance(&grandparent)
                            } else {
                                match x_dir {
                                    Direction::Left => {
                                        x_node.borrow_mut().color = NodeColor::Black;
                                        gp_node.borrow_mut().color = NodeColor::Red;
                                        left_rotate(parent);
                                        right_rotate(grandparent);
                                    }
                                    Direction::Right => {
                                        gp_node.borrow_mut().color = NodeColor::Red;
                                        p_node.borrow_mut().color = NodeColor::Black;
                                        right_rotate(grandparent);
                                    }
                                }
                            }
                        }
                    }else {
                        if let Some(ref u_node) = p_node.borrow_mut().left {
                            if u_node.borrow().color == NodeColor::Red {
                                p_node.borrow_mut().color = NodeColor::Black;
                                u_node.borrow_mut().color = NodeColor::Black;
                                gp_node.borrow_mut().color = NodeColor::Red;
                                insert_balance(&grandparent)
                            } else {
                                match x_dir {
                                    Direction::Left => {
                                        gp_node.borrow_mut().color = NodeColor::Red;
                                        p_node.borrow_mut().color = NodeColor::Black;
                                        left_rotate(grandparent);
                                    }
                                    Direction::Right => {
                                        x_node.borrow_mut().color = NodeColor::Black;
                                        gp_node.borrow_mut().color = NodeColor::Red;
                                        right_rotate(parent);
                                        left_rotate(grandparent);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
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

fn left_rotate(y: &RedBlackTree) {
    let node_y = y.clone();
    if let Some(y_node) = y {
        let z = y_node.borrow_mut().parent.take();
        let x = y_node.borrow_mut().right.take();
        y_node.borrow_mut().parent = x.clone();
        if let Some(x_node) = x {
            let child = x_node.borrow_mut().left.take();
            x_node.borrow_mut().parent = z.clone();
            y_node.borrow_mut().right = child.clone();
            x_node.borrow_mut().left = Some(y_node.clone());
            if let Some(child_node) = child {
                child_node.borrow_mut().parent = Some(y_node.clone());
            }
            if let Some(z_node) = z {
                if z_node.borrow().left == node_y {
                    z_node.borrow_mut().left = Some(x_node);
                } else {
                    z_node.borrow_mut().right = Some(x_node);
                }
            }
        }
    }
}

fn right_rotate(y: &RedBlackTree) {
    let node_y = y.clone();
    if let Some(y_node) = y {
        let z = y_node.borrow_mut().parent.take();
        let x = y_node.borrow_mut().left.take();
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
                if z_node.borrow().left == node_y {
                    z_node.borrow_mut().left = Some(x_node);
                } else {
                    z_node.borrow_mut().right = Some(x_node);
                }
            }
        }
    }
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
    let mut rb_tree = new_rb_tree(42);
    rb_tree = insert_node(rb_tree, 30);
    rb_tree = insert_node(rb_tree, 5000000);
    rb_tree = insert_node(rb_tree, 45);
    rb_tree = insert_node(rb_tree, 55);
    rb_tree = insert_node(rb_tree, 20);
    rb_tree = insert_node(rb_tree, 35);
    rb_tree = insert_node(rb_tree, 3);
    rb_tree = insert_node(rb_tree, 25);
    let print_node = find_key(rb_tree.clone(), 20);
    print_tree(&print_node,0);
    print_tree(&rb_tree.clone(), 0);

    // rb_tree = left_rotate(rb_tree).unwrap();
    
    // println!("{:?}", rb_tree.clone().unwrap().borrow().key);
    // println!(
    //     "{:?}",
    //     get_child(rb_tree.clone(), Direction::Right)
    //         .unwrap()
    //         .borrow()
    //         .key
    // );
    // println!(
    //     "{:?}",
    //     get_child(rb_tree.clone(), Direction::Left)
    //         .unwrap()
    //         .borrow()
    //         .key
    // );
    // println!(
    //     "{:?}",
    //     rb_tree
    //         .clone()
    //         .unwrap()
    //         .borrow()
    //         .right
    //         .clone()
    //         .unwrap()
    //         .borrow()
    //         .key
    // );
    // println!(
    //     "{:?}",
    //     rb_tree
    //         .clone()
    //         .unwrap()
    //         .borrow()
    //         .left
    //         .clone()
    //         .unwrap()
    //         .borrow()
    //         .key
    // );
    // println!(
    //     "{:?}",
    //     get_child(
    //         get_child(rb_tree.clone(), Direction::Left),
    //         Direction::Right
    //     )
    //     .unwrap()
    //     .borrow()
    //     .key
    // );

    // let val = find_key(rb_tree, 50);
    // println!("{:?}", val.unwrap().borrow().key);
    // if let Some(v) = val {
    //     println!("Yay")
    // } else {
    //     println!("Boo")
    // }
    // println!("{:?}", f.clone().unwrap().borrow().key)
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
fn print_tree(rb_tree: &RedBlackTree, cur_level:usize){
    // dfs, with tabs for each level - 1
    if let Some(node) = rb_tree {
        for i in 0..cur_level {
            
            let pad: &str;
            if i == cur_level-1{
                pad = " |→";
            } else {
                pad = " |  ";
            }
            print!("{}",pad.on_white());
        }
        // let _ = std::iter::repeat(print!("-")).take(cur_level);
        let msg = format!(" {} ", node.borrow().key);
        if node.borrow().color == NodeColor::Black {
            println!("{:}", msg.black().on_white());
        } else {
            println!("{:}", msg.red().on_white());
        }
        
        print_tree(&node.borrow().left, cur_level+1);
        print_tree(&node.borrow().right, cur_level+1)
    }
}