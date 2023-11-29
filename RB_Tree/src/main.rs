mod tests;

use RB_Tree::*;
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
    let mut rb_tree = new_rb_tree(10);
    rb_tree = insert(rb_tree, 5);
    rb_tree = insert(rb_tree, 15);
    rb_tree = insert(rb_tree, 3);
    rb_tree = insert(rb_tree, 7);
    rb_tree = insert(rb_tree, 12);
    rb_tree = insert(rb_tree, 18);
    rb_tree = insert(rb_tree, 1);
    rb_tree = insert(rb_tree, 9);
    rb_tree = insert(rb_tree, 14);
    rb_tree = insert(rb_tree, 17);
    rb_tree = insert(rb_tree, 20);
    rb_tree = delete(rb_tree, 10);
    rb_tree = delete(rb_tree, 5);
    rb_tree = delete(rb_tree, 3);
    rb_tree = delete(rb_tree, 7);
    rb_tree = delete(rb_tree, 12);
    rb_tree = delete(rb_tree, 18);
    rb_tree = delete(rb_tree, 1);
    println!("error point");
    rb_tree = delete(rb_tree, 9);
    rb_tree = delete(rb_tree, 14);
    rb_tree = delete(rb_tree, 17);
    rb_tree = delete(rb_tree, 20);
    rb_tree = delete(rb_tree, 15);
    print_tree(&rb_tree, 0);
    // println!("{:?}",check_if_empty(&rb_tree));
    // let mut rep_node;
    // (rb_tree, rep_node) = remove_node(rb_tree, 6);
    // print_tree(&rep_node, 0);
    // Sibling testing:
    //  let (sibling, sib_direction) = get_sibling(find_key(rb_tree.clone(), 334));
    // print_tree(&sibling,0);
    // println!("{:?}", sib_direction)
}


// #[cfg(test)]
// mod tests;