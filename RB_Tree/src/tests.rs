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
    // let rep_node;
    // println!("We goin");
    // (rb_tree, rep_node) = remove_node(rb_tree.clone(),4 );
    // print_tree(&rb_tree, 0);
    // print_tree(&rep_node, 0);
    return rb_tree
}
#[test]
pub fn test_live(){
    assert_eq!(1,1);
}

#[test]
pub fn create_tree_insert(){
    let rb_tree = new_rb_node(4);
    assert!(rb_tree.is_some())
}

pub fn bench_function(elem_num:u32){

}
// #[test]
// pub fn empty_rm_check(){
//     let mut rb_tree = new_rb_tree(1);
//     // rb_tree = (2..=9).into_iter().map(|x| insert(&rb_tree,x)).collect();
//     assert!(check_if_empty(&rb_tree).is_ok());
//     let mut tn;
//     (rb_tree, tn,_) = remove_node(rb_tree, 1);

//     assert!(check_if_empty(&rb_tree).is_err());
//     assert!(tn.is_none());
//     rb_tree = test_tree();


//     (_, tn,_)= remove_node(rb_tree, 4);
//     assert_eq!(tn.unwrap().borrow().key, 5)
    
// }