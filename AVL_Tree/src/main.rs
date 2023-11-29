
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

mod tests;
// TO IMPLEMENT
// 1- DONE: Insert a node to the AVL tree.
// 2- DONE: Delete a node from the AVL tree.
// 3- DONE: Count the number of leaves in a tree.
// 4- DONE: Return the height of a tree.
// 5- DONE: Print In-order traversal of the tree.  -> Starting from root, then all lefts
// 6- DONE: Check if the tree is empty.
// 7- DONE: Print the tree showing its structure. (Using println!(“{:#?}”,tree); is NOT
// sufficient).
// 8- DONE: Rebalance the tree for insert and delete.

use AVL_Tree::*;
fn main() {
    let mut avl_tree = new_avl_tree(12);
    avl_tree = insert(avl_tree, 15);
    avl_tree = insert(avl_tree, 13);
    print_tree(&avl_tree, 0);
    avl_tree = insert(avl_tree, 8);
    avl_tree = insert(avl_tree, 7);
    avl_tree = insert(avl_tree, 6);
    avl_tree = insert(avl_tree, 5);
    avl_tree = insert(avl_tree, 4);
    avl_tree = insert(avl_tree, 3);
    avl_tree = insert(avl_tree, 2);
    avl_tree = insert(avl_tree, 1);
    avl_tree = insert(avl_tree, 11);
    avl_tree = insert(avl_tree, 10);
    avl_tree = insert(avl_tree, 16);
    avl_tree = insert(avl_tree, 14);
    // let leaf_count = count_leaves(&avl_tree);
    // let tree_height = tree_height(&avl_tree);
    // let node_1 = find_key(avl_tree.clone(), 6);
    // let node_2 = find_key(avl_tree.clone(), 37);
    // println!("{}", leaf_count);
    // println!("{}", tree_height);
    let mut keys: Vec<u32> = Vec::new();
    in_order_traversal(&avl_tree, &mut keys);
    println!("{:?}", keys);
    // print_tree(&avl_tree, 0);
    // let result = check_if_empty(&avl_tree);
    print_tree(&avl_tree, 0);
    let mut avl_tree = delete(avl_tree, 14);
    let mut avl_tree = delete(avl_tree, 13);
    // avl_tree = insert(avl_tree, 4);
    print_tree(&avl_tree, 0);
}
