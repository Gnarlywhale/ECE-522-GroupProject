use super::*;

fn test_tree() -> RedBlackTree{
    let mut rb_tree = new_rb_tree(1);
    rb_tree = insert(&rb_tree, 2);
    rb_tree = insert(&rb_tree, 3);
    rb_tree = insert(&rb_tree, 4);
    rb_tree = insert(&rb_tree, 5);
    rb_tree = insert(&rb_tree, 6);
    rb_tree = insert(&rb_tree, 7);
    rb_tree = insert(&rb_tree, 8);
    rb_tree = insert(&rb_tree, 9);
    return rb_tree
}
#[test]
pub fn test_live(){
    assert_eq!(1,1);
}

#[test]
pub fn create_tree_insert(){
    let rb_tree = new_rb_node(4);
    assert!(rb_tree.clone().is_some());
    assert_eq!(rb_tree.clone().unwrap().borrow().key, 4);
    assert_eq!(rb_tree.clone().unwrap().borrow().parent.is_none(), true);
    assert_eq!(rb_tree.clone().unwrap().borrow().left.is_none(), true);
    assert_eq!(rb_tree.clone().unwrap().borrow().right.is_none(), true);
}
#[test]
fn test_ll_rotation() {
    let mut rb_tree = new_rb_tree(3);
    rb_tree = insert(&rb_tree, 2);
    rb_tree = insert(&rb_tree, 1);
    assert_eq!(rb_tree.clone().unwrap().borrow().key, 2);
    assert_eq!(rb_tree.clone().unwrap().borrow().left.clone().is_some(), true);
    assert_eq!(rb_tree.clone().unwrap().borrow().right.clone().is_some(), true);
    assert_eq!(rb_tree.clone().unwrap().borrow().left.clone().unwrap().borrow().key.clone(), 1);
    assert_eq!(rb_tree.clone().unwrap().borrow().right.clone().unwrap().borrow().key.clone(), 3);
}
#[test]
fn test_rr_rotation() {
    let mut rb_tree = new_rb_tree(1);
    rb_tree = insert(&rb_tree, 2);
    rb_tree = insert(&rb_tree, 3);
    assert_eq!(rb_tree.clone().unwrap().borrow().key, 2);
    assert_eq!(rb_tree.clone().unwrap().borrow().left.clone().is_some(), true);
    assert_eq!(rb_tree.clone().unwrap().borrow().right.clone().is_some(), true);
    assert_eq!(rb_tree.clone().unwrap().borrow().left.clone().unwrap().borrow().key.clone(), 1);
    assert_eq!(rb_tree.clone().unwrap().borrow().right.clone().unwrap().borrow().key.clone(), 3);
}

#[test]
fn test_lr_rotation() {
    let mut rb_tree = new_rb_tree(13);
    rb_tree = insert(&rb_tree, 11);
    rb_tree = insert(&rb_tree, 12);
    assert_eq!(rb_tree.clone().unwrap().borrow().key, 12);
    assert_eq!(rb_tree.clone().unwrap().borrow().left.clone().is_some(), true);
    assert_eq!(rb_tree.clone().unwrap().borrow().right.clone().is_some(), true);
    assert_eq!(rb_tree.clone().unwrap().borrow().left.clone().unwrap().borrow().key.clone(), 11);
    assert_eq!(rb_tree.clone().unwrap().borrow().right.clone().unwrap().borrow().key.clone(), 13);
}

#[test]
fn test_rl_rotation() {
    let mut rb_tree = new_rb_tree(12);
    rb_tree = insert(&rb_tree, 15);
    rb_tree = insert(&rb_tree, 13);
    assert_eq!(rb_tree.clone().unwrap().borrow().key, 13);
    assert_eq!(rb_tree.clone().unwrap().borrow().left.clone().is_some(), true);
    assert_eq!(rb_tree.clone().unwrap().borrow().right.clone().is_some(), true);
    assert_eq!(rb_tree.clone().unwrap().borrow().left.clone().unwrap().borrow().key.clone(), 12);
    assert_eq!(rb_tree.clone().unwrap().borrow().right.clone().unwrap().borrow().key.clone(), 15);
}

#[test]
pub fn empty_rm_check(){
    let mut rb_tree = new_rb_tree(1);
    assert!(check_if_empty(&rb_tree).is_ok());
    let mut tn;
    (_,_,_,tn,_,_) = remove_node(rb_tree, 1);

    assert!(check_if_empty(&tn).is_err());

    rb_tree = test_tree();

    (_, _,_,tn,_,_)= remove_node(rb_tree, 4);
    assert_eq!(tn.unwrap().borrow().key, 6)
    
}

#[test]
fn test_find_node() {
    let mut rb_tree = new_rb_tree(9);
    rb_tree = insert(&rb_tree, 8);
    rb_tree = insert(&rb_tree, 7);
    rb_tree = insert(&rb_tree, 6);
    rb_tree = insert(&rb_tree, 5);
    rb_tree = insert(&rb_tree, 4);
    rb_tree = insert(&rb_tree, 3);
    rb_tree = insert(&rb_tree, 2);
    rb_tree = insert(&rb_tree, 1);
    let node = rb_tree.clone().unwrap();
    let mut node_borrow = node.borrow().left.clone().unwrap();
    while let Some(left_child) = node_borrow.clone().borrow().left.clone(){
        node_borrow = left_child.clone();
    }
    assert_eq!(find_key(rb_tree, 1).unwrap().borrow().key, node_borrow.borrow().key);
}

#[test]
fn test_delete_ll_rotation() {
    let mut rb_tree = new_rb_tree(3);
    rb_tree = insert(&rb_tree, 4);
    rb_tree = insert(&rb_tree, 2);
    rb_tree = insert(&rb_tree, 1);
    rb_tree = delete(rb_tree, 4);
    assert_eq!(rb_tree.clone().unwrap().borrow().key, 2);
    assert_eq!(rb_tree.clone().unwrap().borrow().left.clone().is_some(), true);
    assert_eq!(rb_tree.clone().unwrap().borrow().right.clone().is_some(), true);
    assert_eq!(rb_tree.clone().unwrap().borrow().left.clone().unwrap().borrow().key.clone(), 1);
    assert_eq!(rb_tree.clone().unwrap().borrow().right.clone().unwrap().borrow().key.clone(), 3);
}

#[test]
fn test_delete_lr_rotation() {
    let mut rb_tree = new_rb_tree(13);
    rb_tree = insert(&rb_tree, 14);
    rb_tree = insert(&rb_tree, 11);
    rb_tree = insert(&rb_tree, 12);
    rb_tree = delete(rb_tree, 14);
    assert_eq!(rb_tree.clone().unwrap().borrow().key, 12);
    assert_eq!(rb_tree.clone().unwrap().borrow().left.clone().is_some(), true);
    assert_eq!(rb_tree.clone().unwrap().borrow().right.clone().is_some(), true);
    assert_eq!(rb_tree.clone().unwrap().borrow().left.clone().unwrap().borrow().key.clone(), 11);
    assert_eq!(rb_tree.clone().unwrap().borrow().right.clone().unwrap().borrow().key.clone(), 13);
}

#[test]
fn test_height(){
    let tree = test_tree();
    assert_eq!(tree_height(&tree), 4);    
}

#[test]
fn test_leaf_count(){
    let tree = test_tree();
    assert_eq!(count_leaves(&tree), 5);    
}

#[test]
fn test_traversal(){
    let tree = test_tree();
    let mut keys: Vec<u32> = Vec::new();
    in_order_traversal(&tree, &mut keys);
    assert!(keys == (1..10).collect::<Vec<u32>>());
}