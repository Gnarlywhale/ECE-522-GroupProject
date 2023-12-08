
use super::*;
#[allow(dead_code)]
fn test_tree() -> AVLTree {
    let mut avl_tree = new_avl_tree(1);
    avl_tree = insert(&avl_tree, 2);
    avl_tree = insert(&avl_tree, 3);
    avl_tree = insert(&avl_tree, 4);
    avl_tree = insert(&avl_tree, 5);
    avl_tree = insert(&avl_tree, 6);
    avl_tree = insert(&avl_tree, 7);
    avl_tree = insert(&avl_tree, 8);
    avl_tree = insert(&avl_tree, 9);
    return avl_tree;
}
#[test]
fn test_insert_root_node() {
    let avl_tree = new_avl_tree(1);
    assert_eq!(avl_tree.clone().unwrap().borrow().key, 1);
    assert_eq!(avl_tree.clone().unwrap().borrow().parent.is_none(), true);
    assert_eq!(avl_tree.clone().unwrap().borrow().left.is_none(), true);
    assert_eq!(avl_tree.clone().unwrap().borrow().right.is_none(), true);
}

#[test]
fn test_ll_rotation() {
    let mut avl_tree = new_avl_tree(3);
    avl_tree = insert(&avl_tree, 2);
    avl_tree = insert(&avl_tree, 1);
    assert_eq!(avl_tree.clone().unwrap().borrow().key, 2);
    assert_eq!(
        avl_tree.clone().unwrap().borrow().left.clone().is_some(),
        true
    );
    assert_eq!(
        avl_tree.clone().unwrap().borrow().right.clone().is_some(),
        true
    );
    assert_eq!(
        avl_tree
            .clone()
            .unwrap()
            .borrow()
            .left
            .clone()
            .unwrap()
            .borrow()
            .key
            .clone(),
        1
    );
    assert_eq!(
        avl_tree
            .clone()
            .unwrap()
            .borrow()
            .right
            .clone()
            .unwrap()
            .borrow()
            .key
            .clone(),
        3
    );
}

#[test]
fn test_rr_rotation() {
    let mut avl_tree = new_avl_tree(1);
    avl_tree = insert(&avl_tree, 2);
    avl_tree = insert(&avl_tree, 3);
    assert_eq!(avl_tree.clone().unwrap().borrow().key, 2);
    assert_eq!(
        avl_tree.clone().unwrap().borrow().left.clone().is_some(),
        true
    );
    assert_eq!(
        avl_tree.clone().unwrap().borrow().right.clone().is_some(),
        true
    );
    assert_eq!(
        avl_tree
            .clone()
            .unwrap()
            .borrow()
            .left
            .clone()
            .unwrap()
            .borrow()
            .key
            .clone(),
        1
    );
    assert_eq!(
        avl_tree
            .clone()
            .unwrap()
            .borrow()
            .right
            .clone()
            .unwrap()
            .borrow()
            .key
            .clone(),
        3
    );
}

#[test]
fn test_lr_rotation() {
    let mut avl_tree = new_avl_tree(13);
    avl_tree = insert(&avl_tree, 11);
    avl_tree = insert(&avl_tree, 12);
    assert_eq!(avl_tree.clone().unwrap().borrow().key, 12);
    assert_eq!(
        avl_tree.clone().unwrap().borrow().left.clone().is_some(),
        true
    );
    assert_eq!(
        avl_tree.clone().unwrap().borrow().right.clone().is_some(),
        true
    );
    assert_eq!(
        avl_tree
            .clone()
            .unwrap()
            .borrow()
            .left
            .clone()
            .unwrap()
            .borrow()
            .key
            .clone(),
        11
    );
    assert_eq!(
        avl_tree
            .clone()
            .unwrap()
            .borrow()
            .right
            .clone()
            .unwrap()
            .borrow()
            .key
            .clone(),
        13
    );
}

#[test]
fn test_rl_rotation() {
    let mut avl_tree = new_avl_tree(12);
    avl_tree = insert(&avl_tree, 15);
    avl_tree = insert(&avl_tree, 13);
    assert_eq!(avl_tree.clone().unwrap().borrow().key, 13);
    assert_eq!(
        avl_tree.clone().unwrap().borrow().left.clone().is_some(),
        true
    );
    assert_eq!(
        avl_tree.clone().unwrap().borrow().right.clone().is_some(),
        true
    );
    assert_eq!(
        avl_tree
            .clone()
            .unwrap()
            .borrow()
            .left
            .clone()
            .unwrap()
            .borrow()
            .key
            .clone(),
        12
    );
    assert_eq!(
        avl_tree
            .clone()
            .unwrap()
            .borrow()
            .right
            .clone()
            .unwrap()
            .borrow()
            .key
            .clone(),
        15
    );
}

#[test]
fn test_delete_and_empty_tree() {
    let mut avl_tree = new_avl_tree(1);
    avl_tree = delete(avl_tree, 1);
    assert_eq!(avl_tree.clone().is_none(), true);
    assert_eq!(check_if_empty(&avl_tree).is_err(), true);
}

#[test]
fn test_find_node() {
    let mut avl_tree = new_avl_tree(9);
    avl_tree = insert(&avl_tree, 8);
    avl_tree = insert(&avl_tree, 7);
    avl_tree = insert(&avl_tree, 6);
    avl_tree = insert(&avl_tree, 5);
    avl_tree = insert(&avl_tree, 4);
    avl_tree = insert(&avl_tree, 3);
    avl_tree = insert(&avl_tree, 2);
    avl_tree = insert(&avl_tree, 1);
    let node = avl_tree.clone().unwrap();
    let mut node_borrow = node.borrow().left.clone().unwrap();
    while let Some(left_child) = node_borrow.clone().borrow().left.clone() {
        node_borrow = left_child.clone();
    }
    assert_eq!(
        find_key(avl_tree, 1).unwrap().borrow().key,
        node_borrow.borrow().key
    );
}

#[test]
fn test_delete_ll_rotation() {
    let mut avl_tree = new_avl_tree(3);
    avl_tree = insert(&avl_tree, 4);
    avl_tree = insert(&avl_tree, 2);
    avl_tree = insert(&avl_tree, 1);
    avl_tree = delete(avl_tree, 4);
    assert_eq!(avl_tree.clone().unwrap().borrow().key, 2);
    assert_eq!(
        avl_tree.clone().unwrap().borrow().left.clone().is_some(),
        true
    );
    assert_eq!(
        avl_tree.clone().unwrap().borrow().right.clone().is_some(),
        true
    );
    assert_eq!(
        avl_tree
            .clone()
            .unwrap()
            .borrow()
            .left
            .clone()
            .unwrap()
            .borrow()
            .key
            .clone(),
        1
    );
    assert_eq!(
        avl_tree
            .clone()
            .unwrap()
            .borrow()
            .right
            .clone()
            .unwrap()
            .borrow()
            .key
            .clone(),
        3
    );
}

#[test]
fn test_delete_lr_rotation() {
    let mut avl_tree = new_avl_tree(13);
    avl_tree = insert(&avl_tree, 14);
    avl_tree = insert(&avl_tree, 11);
    avl_tree = insert(&avl_tree, 12);
    avl_tree = delete(avl_tree, 14);
    assert_eq!(avl_tree.clone().unwrap().borrow().key, 12);
    assert_eq!(
        avl_tree.clone().unwrap().borrow().left.clone().is_some(),
        true
    );
    assert_eq!(
        avl_tree.clone().unwrap().borrow().right.clone().is_some(),
        true
    );
    assert_eq!(
        avl_tree
            .clone()
            .unwrap()
            .borrow()
            .left
            .clone()
            .unwrap()
            .borrow()
            .key
            .clone(),
        11
    );
    assert_eq!(
        avl_tree
            .clone()
            .unwrap()
            .borrow()
            .right
            .clone()
            .unwrap()
            .borrow()
            .key
            .clone(),
        13
    );
}
#[test]
fn test_height() {
    let tree = test_tree();
    assert_eq!(tree_height(&tree), 4);
}

#[test]
fn test_leaf_count() {
    let tree = test_tree();
    assert_eq!(count_leaves(&tree), 5);
}
#[test]
fn test_traversal() {
    let tree = test_tree();
    let mut keys: Vec<u32> = Vec::new();
    in_order_traversal(&tree, &mut keys);
    assert!(keys == [4, 2, 1, 3, 6, 5, 8, 7, 9]);
}
