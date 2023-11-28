extern crate RB_Tree;
use  RB_Tree::*;

fn main() {
    println!("Hello, world!");
    let rb_tree = RB_Tree::new_rb_tree(4);
    RB_Tree::print_tree(&rb_tree,0)
}
