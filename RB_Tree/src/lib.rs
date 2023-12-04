use colored::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum  NodeColor {
    Red,
    Black,
}
#[derive(Clone, Debug, PartialEq)]
pub enum  Direction {
    Right,
    Left,
    Root,
}
pub type Tree = Rc<RefCell<TreeNode<u32>>>;
pub type RedBlackTree = Option<Tree>;

#[derive(Debug, PartialEq)]
pub struct TreeNode<T> {
    pub color: NodeColor,
    pub key: T,
    pub parent: RedBlackTree,
    pub left: RedBlackTree,
    pub right: RedBlackTree,
}

impl<T: Ord> TreeNode<T> {
    pub fn new(data: T) -> Self {
        Self {
            color: NodeColor::Red,
            key: data,
            parent: None,
            left: None,
            right: None,
        }
    }
}

pub fn new_rb_tree(data: u32) -> RedBlackTree {
    let mut node = TreeNode::new(data);
    node.color = NodeColor::Black;
    Some(Rc::new(RefCell::new(node)))
}
pub fn new_rb_node(data: u32) -> RedBlackTree {
    Some(Rc::new(RefCell::new(TreeNode::new(data))))
}

pub fn find_key(rb_tree: RedBlackTree, data: u32) -> RedBlackTree {
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

pub fn count_leaves(rb_tree: &RedBlackTree) -> u32 {
    if let Some(node) = rb_tree {
        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            return 1;
        } else {
            return count_leaves(&node.borrow().left) + count_leaves(&node.borrow().right);
        }
    } else {
        return 0;
    }
}

pub fn count_nodes(rb_tree: &RedBlackTree) -> u32 {
    if let Some(node) = rb_tree {
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

pub fn in_order_successor(rb_tree: RedBlackTree) -> RedBlackTree {
    if let Some(node) = rb_tree {
        //
        let mut curr_node = node.borrow().right.clone().unwrap();
        while let Some(l_node) = curr_node.clone().borrow().left.clone() {
            curr_node = l_node.clone();
        }
        // Found last successor
        return Some(curr_node.clone());
    }
    None
}

pub fn remove_node(
    rb_tree: RedBlackTree,
    data: u32,
) -> (
    RedBlackTree,
    RedBlackTree,
    NodeColor,
    RedBlackTree,
    RedBlackTree,
    Direction,
) {
    let mut color = NodeColor::Black;
    let mut replacement_node: Option<Rc<RefCell<TreeNode<u32>>>> = None;
    let mut parent_node: Option<Rc<RefCell<TreeNode<u32>>>> = None;
    let (mut sibling, mut sibling_direction) = get_sibling(find_key(rb_tree.clone(), data));
    if let Some(node) = find_key(rb_tree.clone(), data) {
        // Handle terminal node case
        let node_key = node.borrow().key.clone();
        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            color = node.clone().borrow().color.clone();
            let parent = &node.borrow().parent;
            parent_node = parent.clone();
            if let Some(p_node) = parent {
                if node_key < p_node.borrow().key {
                    p_node.borrow_mut().left = None;
                } else {
                    p_node.borrow_mut().right = None;
                }
            } else {
                return (None, None, NodeColor::Black, None, None, Direction::Root);
            }
        } else if node.borrow().left.is_none() && node.borrow().right.is_some() {
            // One right child
            color = node.clone().borrow().color.clone();
            let rep_node = node.borrow().right.clone();
            replacement_node = rep_node.clone();
            let parent = node.borrow().parent.clone();
            parent_node = parent.clone();
            if let Some(replace) = rep_node.clone(){
                replace.clone().borrow_mut().parent = parent.clone();
            }
            if let Some(p_node) = parent {
                if node_key < p_node.borrow().key {
                    p_node.borrow_mut().left = rep_node.clone();
                } else {
                    p_node.borrow_mut().right = rep_node.clone();
                }
            } else {
                node.borrow_mut().key = rep_node.clone().unwrap().borrow().key;
                node.borrow_mut().parent = None;
                node.borrow_mut().left = None;
                node.borrow_mut().right = None
            }
        } else if node.borrow().left.is_some() && node.borrow().right.is_none() {
            // One left child
            color = node.clone().borrow().color.clone();
            let rep_node = node.borrow().left.clone();
            replacement_node = rep_node.clone();
            let parent = node.borrow().parent.clone();
            parent_node = parent.clone();
            if let Some(replace) = rep_node.clone(){
                replace.clone().borrow_mut().parent = parent.clone();
            }
            if let Some(p_node) = parent {
                if node_key < p_node.borrow().key {
                    p_node.borrow_mut().left = rep_node.clone();
                } else {
                    p_node.borrow_mut().right = rep_node.clone();
                }
            } else {
                node.borrow_mut().key = rep_node.clone().unwrap().borrow().key;
                node.borrow_mut().parent = None;
                node.borrow_mut().left = None;
                node.borrow_mut().right = None
            }
        } else {
            let sucessor = in_order_successor(Some(node.clone()));
            // check if sucessor is the right child of the node
            (sibling, sibling_direction) = get_sibling(sucessor.clone());
            if let Some(rep_node) = sucessor.clone() {
                color = rep_node.clone().borrow().color.clone();
                // Set the parent key to match the rep_node key
                let temp_key = rep_node.clone().borrow().key;
                // Remove the reference from the replacement node's parent to the replacement node
                let parent = &rep_node.borrow().parent;
                parent_node = parent.clone();
                if let Some(p_node) = parent.clone() {
                    if rep_node.clone().borrow().key == p_node.clone().borrow().right.clone().unwrap().borrow().key {
                        // Successor is right child, bring right child along during replacement
                        if let Some(right) = rep_node.clone().borrow().right.clone(){
                            right.clone().borrow_mut().parent = parent.clone();
                            replacement_node = Some(right);
                        }
                        node.clone().borrow_mut().right = rep_node.clone().borrow().right.clone();
            
                        } else {
                            // Successor is a left descendant, set its right child to be be the rep_node's parent left child
                            if let Some(right) = rep_node.clone().borrow().right.clone(){
                                right.clone().borrow_mut().parent = parent.clone();
                                replacement_node = Some(right);
                            }
                            rep_node.clone().borrow().parent.clone().unwrap().borrow_mut().left = rep_node.clone().borrow().right.clone(); 
                        }
                    if rep_node.clone().borrow().key < p_node.clone().borrow().key {
                        node.borrow_mut().key = temp_key;
                    } else {
                        node.borrow_mut().key = temp_key;
                    }
                }
            }
        }
    }

    return (
        rb_tree,
        replacement_node,
        color,
        parent_node,
        sibling,
        sibling_direction,
    );
}

pub fn delete(rb_tree: RedBlackTree, data: u32) -> RedBlackTree {
    let (new_tree, replacement, color, parent, sibling, sibling_direction) =
        remove_node(rb_tree, data);
    if new_tree.is_some() {
        let root = delete_balance(replacement, color, parent, sibling, sibling_direction);
        if root.is_some(){
            return root;
        }
    }
    return new_tree;
}

pub fn delete_balance(replacement: RedBlackTree, org_color: NodeColor, parent: RedBlackTree, sibling: RedBlackTree, sibling_direction: Direction) -> RedBlackTree {
    if let Some(r_node) = replacement.clone() {
        //simple recolor case
        if r_node.borrow().color == NodeColor::Red
            || org_color == NodeColor::Red
            || r_node.borrow().parent.is_none()
        {
            println!("simple case");
            r_node.borrow_mut().color = NodeColor::Black;
            return None;
        }
    }
    //simple recolor case
    else {
        if org_color == NodeColor::Red {
            println!("simple case None replacement");
            return None;
        }
    }
    if let Some(s_node) = sibling.clone() {
        let mut s_right_color = NodeColor::Black;
        let mut s_left_color = NodeColor::Black;
        if let Some(s_right) = s_node.clone().borrow().right.clone() {
            s_right_color = s_right.borrow().color.clone();
        }
        if let Some(s_left) = s_node.clone().borrow().left.clone() {
            s_left_color = s_left.borrow().color.clone();
        }
        if s_node.borrow().color == NodeColor::Black {
            if s_right_color == NodeColor::Black && s_left_color == NodeColor::Black {
                //recur for parent
                if let Some(p_node) = parent.clone(){
                    if p_node.clone().borrow().color == NodeColor::Red{
                        println!("red parent");
                        p_node.clone().borrow_mut().color = NodeColor::Black;
                        s_node.clone().borrow_mut().color = NodeColor::Red;
                        return None;
                    }else{
                        println!("recursive case");
                        s_node.clone().borrow_mut().color = NodeColor::Red;
                        let (new_s, new_s_direction) = get_sibling(parent.clone());
                        let new_parent = p_node.clone().borrow().parent.clone();
                        return delete_balance(parent, NodeColor::Black, new_parent, new_s, new_s_direction);
                    }
                }
            } else {
                match sibling_direction {
                    Direction::Left => {
                        if s_left_color == NodeColor::Red {
                            println!("left left case");
                            //left left case
                            let mut p_color = NodeColor::Black;
                            if let Some(p_node) = parent.clone(){
                                p_color = p_node.clone().borrow().color.clone();
                                p_node.clone().borrow_mut().color = NodeColor::Black;
                            }
                            if let Some(s_left) = s_node.clone().borrow().left.clone() {
                                s_left.clone().borrow_mut().color = NodeColor::Black;
                            }
                            s_node.clone().borrow_mut().color = p_color;
                            return right_rotate(&parent);
                        } else if s_right_color == NodeColor::Red {
                            println!("left right case");
                            //left right case
                            let mut p_color = NodeColor::Black;
                            if let Some(p_node) = parent.clone(){
                                p_color = p_node.clone().borrow().color.clone();
                                p_node.clone().borrow_mut().color = NodeColor::Black;
                            }
                            if let Some(s_right) = s_node.clone().borrow().right.clone() {
                                s_right.clone().borrow_mut().color = p_color;
                            }
                            s_node.clone().borrow_mut().color = NodeColor::Black;
                            left_rotate(&sibling);
                            return right_rotate(&parent);
                        }
                    }
                    Direction::Right => {
                        if s_right_color == NodeColor::Red {
                            println!("right right case");
                            //right right case
                            let mut p_color = NodeColor::Black;
                            if let Some(p_node) = parent.clone(){
                                p_color = p_node.clone().borrow().color.clone();
                                p_node.clone().borrow_mut().color = NodeColor::Black;
                            }
                            if let Some(s_right) = s_node.clone().borrow().right.clone() {
                                s_right.clone().borrow_mut().color = NodeColor::Black;
                            }
                            s_node.clone().borrow_mut().color = p_color;
                            return left_rotate(&parent);
                        } else if s_left_color == NodeColor::Red {
                            println!("right left case");
                            //right left case
                            let mut p_color = NodeColor::Black;
                            if let Some(p_node) = parent.clone(){
                                p_color = p_node.clone().borrow().color.clone();
                                p_node.clone().borrow_mut().color = NodeColor::Black;
                            }
                            if let Some(s_left) = s_node.clone().borrow().left.clone() {
                                s_left.clone().borrow_mut().color = p_color;
                            }
                            s_node.clone().borrow_mut().color = NodeColor::Black;
                            right_rotate(&sibling);
                            return left_rotate(&parent);
                        }
                    }
                    Direction::Root => {}
                }
            }
        } else if s_node.borrow().color == NodeColor::Red {
            match sibling_direction{
                Direction::Left=>{
                    println!("red sibling left case");
                    //left case if sibling is red
                    let mut new_sibling = None;
                    if let Some(p_node) = parent.clone(){
                        p_node.clone().borrow_mut().color = NodeColor::Red;
                        s_node.clone().borrow_mut().color = NodeColor::Black;
                    }
                    let rotated = right_rotate(&parent);
                    if let Some(p_node) = parent.clone(){
                        new_sibling = p_node.clone().borrow().left.clone();
                    }
                    let recurred = delete_balance(replacement, NodeColor::Black, parent, new_sibling, Direction::Right);
                    if recurred.is_some(){
                        return recurred;
                    }else{
                        return rotated;
                    }
                }
                Direction::Right=>{
                    println!("red sibling right case");
                    //right case if sibling is red
                    let mut new_sibling = None;
                    if let Some(p_node) = parent.clone(){
                        p_node.clone().borrow_mut().color = NodeColor::Red;
                        s_node.clone().borrow_mut().color = NodeColor::Black;
                    }
                    let rotated = left_rotate(&parent);
                    if let Some(p_node) = parent.clone(){
                        new_sibling = p_node.clone().borrow().right.clone();
                    }
                    let recurred = delete_balance(replacement, NodeColor::Black, parent, new_sibling, Direction::Right);
                    if recurred.is_some(){
                        return recurred;
                    }else{
                        return rotated;
                    }
                }
                Direction::Root=>{}
            }
        }
    } else {
        //recur for parent
        println!("recursive no other nodes");
        if let Some(p_node) = parent.clone(){
            if p_node.clone().borrow().color == NodeColor::Red{
                p_node.clone().borrow_mut().color = NodeColor::Black;
                return None;
            }else{
                let (new_s, new_s_direction) = get_sibling(parent.clone());
                let new_parent = p_node.clone().borrow().parent.clone();
                return delete_balance(parent, NodeColor::Black, new_parent, new_s, new_s_direction);
            }
        }
    }
    None
}

pub fn insert_node(rb_tree: &RedBlackTree, data: u32) -> RedBlackTree {
    if let Some(node) = rb_tree.clone() {
        if data == node.borrow().key {
            return None;
        }
        if data < node.borrow().key {
            let left = node.borrow_mut().left.take();
            let new_left = insert_node(&left, data);
            node.borrow_mut().left = new_left;
            if let Some(left_node) = &node.borrow().left {
                left_node.borrow_mut().parent = Some(node.clone());
            }
        } else {
            let right = node.borrow_mut().right.take();
            let new_right = insert_node(&right, data);
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

pub fn insert(rb_tree: &RedBlackTree, data: u32) -> RedBlackTree {
    let new_tree = insert_node(rb_tree, data);
    let node = &find_key(new_tree.clone(), data);
    let rotates = insert_balance(node);
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
            Direction::Root => {}
        }
    }
    return new_tree;
}
pub fn in_order_traversal(rb_tree: &RedBlackTree, keys: &mut Vec<u32>) {
    if let Some(node) = rb_tree {
        let node_borrow = node.borrow();
        in_order_traversal(&node_borrow.left, keys);
        keys.push(node_borrow.key);
        in_order_traversal(&node_borrow.right, keys);
    }
}
pub fn insert_balance(x: &RedBlackTree) -> Vec<(Direction, u32)> {
    if let Some(x_node) = x {
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
                        Direction::Root => {}
                        Direction::Left => {
                            let u = gp_node.clone().borrow().right.clone();
                            if let Some(ref u_node) = u {
                                if u_node.borrow().color == NodeColor::Red {
                                    p_node.clone().borrow_mut().color = NodeColor::Black;
                                    u_node.borrow_mut().color = NodeColor::Black;
                                    gp_node.borrow_mut().color = NodeColor::Red;
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
                                        Direction::Root => {}
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
                                    Direction::Root => {}
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
                                    return insert_balance(&p_node.clone().borrow().parent);
                                } else {
                                    match x_dir {
                                        Direction::Root => {}
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
                                    Direction::Root => {}
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

pub fn get_child(opt_node: RedBlackTree, direction: Direction) -> RedBlackTree {
    if let Some(node) = opt_node {
        match direction {
            Direction::Root => {}
            Direction::Right => return node.borrow().right.clone(),
            Direction::Left => return node.borrow().left.clone(),
        }
    }
    None
}
pub fn get_sibling(opt_node: RedBlackTree) -> (RedBlackTree, Direction) {
    if let Some(node) = opt_node {
        let parent = &node.borrow().parent;
        if let Some(p_node) = parent {
            if node.borrow().key < p_node.borrow().key {
                return (p_node.borrow().right.clone(), Direction::Right);
            } else {
                return (p_node.borrow().left.clone(), Direction::Left);
            }
        }
    }
    return (None, Direction::Root);
}

pub fn left_rotate(y: &RedBlackTree) -> RedBlackTree {
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
pub fn tree_height(tree: &Option<Tree>) -> i32 {
    if let Some(node) = tree {
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
pub fn right_rotate(y: &RedBlackTree) -> RedBlackTree {
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

// TO IMPLEMENT
// 1- Insert a node to the red-black tree.
// 2- Delete a node from the red-black tree.
// 3- Count the number of leaves in a tree.
// 4- Return the height of a tree.
// 5- Print In-order traversal of the tree.  -> Starting from root, then all lefts
// 6- Check if the tree is empty.
// 7- Print the tree showing its colors and structure. (Using println!(“{:#?}”,tree); is NOT
// sufficient)

//
// Swap commented code to allow null prints to help clarify which children are left and right nodes
// Could probably implement both versions as an added feature
pub fn print_tree(rb_tree: &RedBlackTree, cur_level: usize) {
    for i in 0..cur_level {

        let pad: &str;
        if i == cur_level-1{
            pad = " |→";
        } else {
            pad = " |  ";
        }
        print!("{}",pad.on_white());
    }
    // dfs, with tabs for each level - 1
    if let Some(node) = rb_tree {
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
        if node.borrow().color == NodeColor::Black {
            println!("{:}", msg.black().on_white());
        } else {
            println!("{:}", msg.red().on_white());
        }
        print_tree(&node.borrow().left, cur_level + 1);
        print_tree(&node.borrow().right, cur_level + 1)
    } else {
        println!();
    }
}

pub fn check_if_empty(tree: &RedBlackTree) -> Result<(),()> {
    if tree.is_some() {
        return Ok(())
    }
    else {
        return Err(())
    }
}

// pub fn parse_tree(file_path:String,mut rb_tree: RedBlackTree) -> RedBlackTree{
    
//     return  rb_tree;
// }
// #[cfg(test)]
// mod tests;