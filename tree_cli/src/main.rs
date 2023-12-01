extern crate RB_Tree;
extern crate AVL_Tree;
use  RB_Tree::*;
use AVL_Tree::*;

use rfd::FileDialog;
use std::fs::File;
use std::io::{self, BufRead, Write};
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
                if let Ok(lines) = read_lines(fpath) {
                    for line in lines {
                        if let Ok(ip) = line {
                            println!("{}", ip);
                        }
                    }
                }
            } else {
                println!("No file selected. Have a great one!")
            }
            },
        "c" => println!("Create a tree"),
        "q" => {
            println!("Have a great day!");
            return
        }
        _ => println!("Invalid input, see above")
    }
    // println!("Hello, world!");
    // let rb_tree = RB_Tree::new_rb_tree(4);
    // RB_Tree::print_tree(&rb_tree,0)
}
