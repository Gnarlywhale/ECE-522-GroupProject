use binary_lib::*;
use std::cell::RefCell;
use std::rc::Rc;

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

fn find_key(rb_tree: RedBlackTree, key: u32) -> Option<Rc<RefCell<TreeNode<u32>>>> {
    if let Some(node) = rb_tree {
        if key == node.borrow_mut().key {
            return Some(node);
        } else if key < node.borrow_mut().key {
            let left = node.borrow_mut().left.take();
            return find_key(left, key);
        } else {
            let right = node.borrow_mut().right.take();
            return find_key(right, key);
        }
    }
    None
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

fn get_child(opt_node: RedBlackTree, direction: Direction) -> RedBlackTree {
    if let Some(node) = opt_node {
        match direction {
            Direction::Right => return node.borrow().right.clone(),
            Direction::Left => return node.borrow().left.clone(),
        }
    }
    None
}

fn left_rotate(y: RedBlackTree) -> Result<RedBlackTree, ()> {
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
                if z_node.borrow().left == node_y{
                    z_node.borrow_mut().left = Some(x_node);
                }
                else{
                    z_node.borrow_mut().right = Some(x_node);
                }
                Ok(Some(z_node))
            } else {
                Ok(Some(x_node))
            }
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}

fn right_rotate(y: RedBlackTree) -> Result<RedBlackTree, ()> {
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
                child_node.borrow_mut().parent = Some(y_node);
            }
            if let Some(z_node) = z {
                if z_node.borrow().left == node_y{
                    z_node.borrow_mut().left = Some(x_node);
                }
                else{
                    z_node.borrow_mut().right = Some(x_node);
                }
                Ok(Some(z_node))
            } else {
                Ok(Some(x_node))
            }
        } else {
            Err(())
        }
    } else {
        Err(())
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
    rb_tree = insert_node(rb_tree, 3);
    rb_tree = insert_node(rb_tree, 50);
    rb_tree = insert_node(rb_tree, 45);
    rb_tree = insert_node(rb_tree, 55);
    rb_tree = left_rotate(rb_tree).unwrap();
    println!("{:?}", rb_tree.clone().unwrap().borrow().key);
    println!(
        "{:?}",
        get_child(rb_tree.clone(), Direction::Right)
            .unwrap()
            .borrow()
            .key
    );
    println!(
        "{:?}",
        get_child(rb_tree.clone(), Direction::Left)
            .unwrap()
            .borrow()
            .key
    );
    println!(
        "{:?}",
        rb_tree
            .clone()
            .unwrap()
            .borrow()
            .right
            .clone()
            .unwrap()
            .borrow()
            .key
    );
    println!(
        "{:?}",
        rb_tree
            .clone()
            .unwrap()
            .borrow()
            .left
            .clone()
            .unwrap()
            .borrow()
            .key
    );
    println!(
        "{:?}",
        get_child(
            get_child(rb_tree.clone(), Direction::Left),
            Direction::Right
        )
        .unwrap()
        .borrow()
        .key
    );

    let val = find_key(rb_tree, 50);
    println!("{:?}", val.unwrap().borrow().key);
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
