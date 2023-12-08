extern crate rb_tree;
extern crate avl_tree;
use avl_tree::*;
use rb_tree::*;

use rfd::FileDialog;
use std::fs::File;
use std::io::{self, BufRead, Write, BufReader, Lines};
use std::path::Path;



// From: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn combine_rb_tree(mut tree: RedBlackTree) -> RedBlackTree{
    println!("Select 2nd tree");
    let file = FileDialog::new()
    .add_filter("text", &["txt"])
    .set_directory("/")
    .pick_file();
            if let Some(fpath) = file {
                // load the file
                if let Ok(mut lines) = read_lines(fpath) {                    
                    match lines.next().unwrap().unwrap().as_str() {
                        "RB_Tree" | "AVL_Tree" => {
                            tree = parse_rb_tree(tree.clone(), lines);
                        }
                        _ => {
                            println!("Coud not parse 2nd tree");
                        }                        
                    }
                }
                
            } 
            return tree;
}

fn combine_avl_tree(mut tree: AVLTree) -> AVLTree{
    println!("Select 2nd tree");
    let file = FileDialog::new()
    .add_filter("text", &["txt"])
    .set_directory("/")
    .pick_file();
            if let Some(fpath) = file {
                // load the file
                if let Ok(mut lines) = read_lines(fpath) {                    
                    match lines.next().unwrap().unwrap().as_str() {
                        "RB_Tree" | "AVL_Tree" => {
                            tree = parse_avl_tree(tree.clone(), lines);
                        }
                        _ => {
                            println!("Coud not parse 2nd tree");
                        }                        
                    }
                }
                
            } 
            return tree;
}
fn main() {

    let mut input = String::new();
    println!("Welcome to Arborium, the very professional and hyper-efficient Red-Black & AVL Tree builder 🌳, now in Rust™!");
    println!("Valid input is shown in parentheses.");
    println!("Would you like to");
    println!("(l)oad a tree?");
    println!("(c)ombine two trees? (Type of first tree will be used for output tree)");
    println!("Create a (n)ew tree?");
    println!("or (q)uit?");
    
    io::stdout().flush().expect("Cannot flush stdout");

    io::stdin()
        .read_line(&mut input)
        .expect("Cannot read user input");

    match input.trim() {
        "l" => {
            println!("Loading a tree");
            let file = FileDialog::new()
    .add_filter("text", &["txt"])
    .set_directory("/")
    .pick_file();
// println!("{:?}",file)
            if let Some(fpath) = file {
                // load the file
                if let Ok(mut lines) = read_lines(fpath) {
                    match lines.next().unwrap().unwrap().as_str() {
                        "RB_Tree" => {
                            let rb:RedBlackTree = parse_rb_tree(None, lines);
                            rb_tree::print_tree(&rb, 0);
                            rb_interface(rb)
                        }
                        "AVL_Tree" => {
                            let avl = parse_avl_tree(None, lines);
                            avl_tree::print_tree(&avl, 0);
                            avl_interface(avl)
                        }
                        _ => {}
                    }
                    // for line in lines {
                    //     if let Ok(ip) = line {
                    //         println!("{}", ip);
                    //     }
                    // }
                }
            } else {
                println!("No file selected. Have a great one!")
            }
            },
        "c" => {
            
            println!("Loading a tree");
            let file = FileDialog::new()
    .add_filter("text", &["txt"])
    .set_directory("/")
    .pick_file();
// println!("{:?}",file)
            if let Some(fpath) = file {
                // load the file
                if let Ok(mut lines) = read_lines(fpath) {
                    match lines.next().unwrap().unwrap().as_str() {
                        "RB_Tree" => {
                            let mut rb = parse_rb_tree(None, lines);
                            rb = combine_rb_tree(rb);
                            rb_tree::print_tree(&rb, 0);
                            rb_interface(rb)
                        }
                        "AVL_Tree" => {
                            let mut avl = parse_avl_tree(None, lines);
                            avl = combine_avl_tree(avl);
                            avl_tree::print_tree(&avl, 0);
                            avl_interface(avl)
                        }
                        _ => {}
                    }
                }
                
            } else {
                println!("No file selected. Have a great one!")
            }
        }
        "n" =>{
            println!("Would you like to create a Red-Black tree (rb) or an AVL tree(avl)?");
            input = "".to_string();
            io::stdin()
            .read_line(&mut input)
            .expect("Cannot read user input");
            match input.trim(){
                "rb" => {
                    rb_interface(None)
                }
                "avl" => {
                    avl_interface(None)
                }
                _ => println!("{} Invalid input, see above and try again.", input.trim())
            }
        },
        "q" => {
            println!("Have a great day!");
            return
        }
        _ => println!("Invalid input, see above and try again.")
    }
    // println!("Hello, world!");
    // let rb_tree = rb_tree::new_rb_tree(4);
    // rb_tree::print_tree(&rb_tree,0)
}

fn parse_input() -> String{
    let mut input = String::new();

    
    io::stdout().flush().expect("Cannot flush stdout");

    io::stdin()
        .read_line(&mut input)
        .expect("Cannot read user input");
    return input;
}
// 1- Insert a node to the red-black tree.
// 2- Delete a node from the red-black tree.
// 3- Count the number of leaves in a tree.
// 4- Return the height of a tree.
// 5- Print In-order traversal of the tree.  -> Starting from root, then all lefts
// 6- Check if the tree is empty.
// 7- Print the tree showing its colors and structure. (Using println!(“{:#?}”,tree); is NOT
// sufficient)
fn list_commands(){
    println!("(i)nsert a new node: i 42"); 
    println!("(d)elete a node: d 42"); 
    println!("(c)ount the number of leaves");
    println!("Display the (h)eight of the tree"); 
    println!("Display the in-order (t)raversal of the tree");
    println!("Check if the tree is (e)mpty"); 
    println!("(p)rint the tree");
    println!("(s)ave the tree");
    println!("(q)uit Arborium");
}

// fn get_input_data(cmd:String) -> Result<u32, String>{
//     let elems = cmd.split(" ").collect::<Vec<&str>>();
//     println!("i l {}",elems.len());
//     if elems.len() != 2 {
//         return Err(format!("Invalid number of inputs. Expected 2, got {}", elems.len())); 

//     }
//     if let Ok(data) = elems[1].trim().parse::<u32>() {
//         return Ok(data);
//     } else {
//         return Err(("Invalid numeric input. Please try again.").to_string());

//     }
    
// }
// 'i' => {
//     match get_input_data(cmd){
//         Ok(data) => {avl_tree::insert(&tree, data);}
//         Err(msg) => {println!("{}",msg);}
//     } 
// }
// 'd' => {
//     match get_input_data(cmd){
//         Ok(data) => {avl_tree::delete(tree.clone(), data);}
//         Err(msg) => {println!("{}",msg);}
//     } 
// }
#[allow(unused_assignments)]
fn get_input_data(cmd:String) -> Result<u32, String>{
    let elems = cmd.split(" ").collect::<Vec<&str>>();
    let mut errmsg;    
    if elems.len() != 2 {
        errmsg = format!("Invalid number of inputs. Expected 2, got {}", elems.len()); 
        // TODO make this actually fail
    }
    if let Ok(data) = elems[1].trim().parse::<u32>() {
        return Ok(data);
    } else {
        errmsg = "Invalid numeric input. Please try again.".to_string();

    }
    return Err(errmsg)
}
fn rb_interface(mut tree:RedBlackTree){
    loop {
    println!("Please enter a command [(o) for a list of options]");
    let cmd = parse_input();

    match cmd.trim().chars().next().unwrap(){
        'p' => {rb_tree::print_tree(&tree,0)},
        'i' => {
            match  get_input_data(cmd) {
                Ok(data) => {tree = rb_tree::insert(&tree, data);}
                Err(msg) => {println!("{}",msg)}
            }            
        }
        'd' => {
            match  get_input_data(cmd) {
                Ok(data) => {tree = rb_tree::delete(tree, data);}
                Err(msg) => {println!("{}",msg)}
            }
        }
        'q' => {
            println!("Hope you've enjoyed your arbory experience, goodbye 👋");
            break
        }
        'h' => {
            println!("Tree height: {}", rb_tree::tree_height(&tree))
        }
        'c' => {
            println!("Number of leaves: {}", rb_tree::count_leaves(&tree))
        }
        't' => {
            let mut keys: Vec<u32> = Vec::new();
            rb_tree::in_order_traversal(&tree, &mut keys);
            println!("{:?}",keys);
        }
        'e' => {
            match rb_tree::check_if_empty(&tree) {
                Ok(()) => {println!("Tree is not empty")}
                Err(()) => {println!("Tree is empty")}
            }
            
        }
        's' => {
            println!("Pick file path/name");
            let path = std::env::current_dir().unwrap();
            
            let file = FileDialog::new()
            .set_file_name("rb_tree.txt")
            .set_directory(&path)            
            .save_file();

            if let Some(file) = file {
                // Write some data to the file
                let mut f = File::create(file).unwrap();
                let _ = f.write(b"RB_Tree\r\n");
                let mut keys: Vec<u32> = Vec::new();
                rb_tree::in_order_traversal(&tree, &mut keys);
                for k in keys {
                    let val = k.to_string() + "\r\n";
                    let _ = f.write(val.as_bytes());
                }
                f.flush().unwrap();
                println!("Tree saved!");
            } else {
                println!("No file selected, aborting.")
            }
            
        }
        'o' => {
            list_commands();
        }
        _ => {
            println!("Invalid command");
            list_commands()
        }
    }
    }
}
fn avl_interface(mut tree: AVLTree){
    loop {
    println!("Please enter a command [(o) for a list of options]");
    let cmd = parse_input();

    match cmd.trim().chars().next().unwrap(){
        'p' => {avl_tree::print_tree(&tree,0)},
        'i' => {
            match  get_input_data(cmd) {
                Ok(data) => {tree = avl_tree::insert(&tree, data);}
                Err(msg) => {println!("{}",msg)}
            }            
        }
        'd' => {
            match  get_input_data(cmd) {
                Ok(data) => {tree = avl_tree::delete(tree, data);}
                Err(msg) => {println!("{}",msg)}
            }
        }
        'q' => {
            println!("Hope you've enjoyed your arbory experience, goodbye 👋");
            break
        }
        'h' => {
            println!("Tree height: {}", avl_tree::tree_height(&tree))
        }
        'c' => {
            println!("Number of leaves: {}", avl_tree::count_leaves(&tree))
        }
        't' => {
            let mut keys: Vec<u32> = Vec::new();
            avl_tree::in_order_traversal(&tree, &mut keys);
            println!("{:?}",keys);
        }
        'e' => {
            match  avl_tree::check_if_empty(&tree) {
                Ok(()) => {println!("Tree is not empty")}
                Err(()) => {println!("Tree is empty")}
            }
            
        }
        's' => {
            println!("Pick file path/name");
            let path = std::env::current_dir().unwrap();
            
            let file = FileDialog::new()
            .set_file_name("avl_tree.txt")
            .set_directory(&path)            
            .save_file();

            if let Some(file) = file {
                // Write some data to the file
                let mut f = File::create(file).unwrap();
                let _ = f.write(b"AVL_Tree\r\n");
                let mut keys: Vec<u32> = Vec::new();
                avl_tree::in_order_traversal(&tree, &mut keys);
                for k in keys {
                    let val = k.to_string() + "\r\n";
                    let _ = f.write(val.as_bytes());
                }
                f.flush().unwrap();
                
                println!("Tree saved!");
            } else {
                println!("No file selected, aborting.")
            }
        }
        'o' => {
            list_commands();
        }
        _ => {
            println!("Invalid command");
            list_commands()
        }
    }
    }
}

// TODO error handling
fn parse_rb_tree(mut rb: RedBlackTree, lines:Lines<BufReader<File>>) -> RedBlackTree {
    for line in lines {
        rb = rb_tree::insert(&rb, line.unwrap().parse().unwrap());
    }
    return rb
    
}

fn parse_avl_tree(mut avl: AVLTree, lines:Lines<BufReader<File>>) -> AVLTree{
    for line in lines {
        avl = avl_tree::insert(&avl, line.unwrap().parse().unwrap());
    }
    return avl
    
}