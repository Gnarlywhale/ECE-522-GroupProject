extern crate RB_Tree;
extern crate AVL_Tree;
use  RB_Tree::*;

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


fn main() {

    let mut input = String::new();
    println!("Welcome to the very professional and hyper efficient Red-Black & AVL Tree builder, now in Rust™!");
    println!("Valid input is shown in parentheses.");
    println!("Would you like to");
    println!("(l)oad a tree?");
    println!("(c)reate a new tree?");
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
                            let rb = parse_rb_tree(lines);
                            RB_Tree::print_tree(&rb, 0);
                            rb_interface(rb)
                        }
                        "AVL_Tree" => {}
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
        "c" =>{
            println!("Would you like to create a Red-Black tree (rb) or an AVL tree(avl)?");
            input = "".to_string();
            io::stdin()
            .read_line(&mut input)
            .expect("Cannot read user input");
            match input.trim(){
                "rb" => {
                    rb_interface(None)
                }
                "avl" => {println!("Create AVL!")}
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
    // let rb_tree = RB_Tree::new_rb_tree(4);
    // RB_Tree::print_tree(&rb_tree,0)
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
    println!("(s)ave the tree"); // todo
}

fn get_input_data(cmd:String) -> Result<u32, ()>{
    let elems = cmd.split(" ").collect::<Vec<&str>>();
            
    if elems.len() != 2 {
        println!("Invalid number of inputs. Expected 2, got {}", elems.len()); 

    }
    if let Ok(data) = elems[1].trim().parse::<u32>() {
        return Ok(data);
    } else {
        println!("Invalid numeric input. Please try again.");

    }
    return Err(())
}
fn rb_interface(mut rb:RedBlackTree){
    loop {
    println!("Please enter a command [(o) for a list of options]");
    let cmd = parse_input();

    match cmd.trim().chars().next().unwrap(){
        'p' => {RB_Tree::print_tree(&rb,0)},
        'i' => {
            if let Ok(data) = get_input_data(cmd){
                rb = RB_Tree::insert(&rb, data)
            }
            
        }
        'd' => {
            if let Ok(data) = get_input_data(cmd){
                rb = RB_Tree::delete(rb, data)
            } 
        }
        'q' => {
            println!("Hope you've enjoyed your arbory experience, goodbye 👋");
            break
        }
        'h' => {
            println!("Tree height: {}", RB_Tree::tree_height(&rb))
        }
        't' => {
            let mut keys: Vec<u32> = Vec::new();
            RB_Tree::in_order_traversal(&rb, &mut keys);
            println!("{:?}",keys);
        }
        'e' => {
            match check_if_empty(&rb) {
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
                RB_Tree::in_order_traversal(&rb, &mut keys);
                for k in keys {
                    let val = k.to_string() + "\r\n";
                    let _ = f.write(val.as_bytes());
                }
                f.flush().unwrap();
                
            }
            println!("Tree saved!");
        }
        _ => {
            println!("Invalid command");
            list_commands()
        }
    }
    }
}

// TODO error handling
fn parse_rb_tree(lines:Lines<BufReader<File>>) -> RedBlackTree {
    let mut rb= None;
    for line in lines {
        rb = RB_Tree::insert(&rb, line.unwrap().parse().unwrap());
    }
    return rb
    
}