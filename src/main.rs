extern crate rand;

mod wordpositions;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use std::process;
use std::collections::HashMap;
use wordpositions::WordPositions;

fn main() {
  let args : Vec<String> = env::args().collect();
  if args.len() < 2 {
    println!("Enter a filename to read\nUsage: rs333 [FILE]");
    process::exit(1);
  }
  let filename = &args[1];
  
  // Collect the contents, quit if it's not ASCII
  let contents = match read_file(&filename) {
    None => { println!("File is not ASCII, exiting");
              return;
            },
    Some(x) => x,
  };
  
  // Create the word HT
  //let wordmap : HashMap<&str, WordPositions> = HashMap::new();
  
  println!("Contents of {} is:\n{}", filename, contents);
}

fn read_file(pathstr : &str) -> Option<String> {
  let path = Path::new(pathstr);
  
  let mut file = match File::open(&path) {
    Err(why) => panic!("Couldn't open {}: {}", path.display(),
                why.description()),
    Ok(file) => file,
  };
  
  let mut contents = String::new();
  match file.read_to_string(&mut contents) {
    Err(why) => panic!("Couldn't open {}: {}", path.display(),
                why.description()),
    Ok(_) => {},
  }
  
  if !contents.is_ascii() {
    return None
  }
  Some(contents)
}
