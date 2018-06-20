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
  let filename : &String = &args[1];
  
  // Collect the contents, quit if it's not ASCII
  let contents : String = match read_file(&filename) {
    None => { println!("File is not ASCII, exiting");
              return;
            },
    Some(x) => x,
  };
  println!("Contents of {} is:\n{}", filename, contents);
  
  // Create the word HT
  let mut wordmap : HashMap<&str, WordPositions> = HashMap::new();
  
  loop_and_insert(&mut wordmap, &contents);
  
  for (word, wp) in wordmap {
    println!("{} : {}", word, wp.num());
  }
}

// Reads `contents` and for each word, places a new position into the 
// WordPositions for at the key corresponding to that word
fn loop_and_insert<'a>(map : &mut HashMap<&'a str, WordPositions>, 
                    contents : &'a String) {
  let split: Vec<& str> =  contents
                            .split(|c:char| !c.is_alphanumeric())
                              // ^ returns a Split<'a>
                            .collect();  // collect() creates a new collection 
                                         // that has its own lifetime

  // Look at each alphabetic word, and put its position in the corresponding
  // value
  for token in split {
    // Do not record empty strings
    if token.len() == 0 { continue; }
    // Avoid null dereferences by inserting a new WP
    if !map.contains_key(token) {
      map.insert(token, WordPositions::new());
    }
    // Add one to the number of instances of this word found
    match map.get_mut(token) {
      Some(wp) => { wp.inc();
                    wp.add(3);  // FIXME: append the actual position
                  },
      None => panic!("map should have already had a value"),
    };
  }
}

fn read_file(pathstr : &str) -> Option<String> {
  let path : &Path = Path::new(pathstr);
  
  let mut file = match File::open(&path) {
    Err(why) => panic!("Couldn't open {}: {}", path.display(),
                  why.description()),
    Ok(_file) => _file,
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
  // Because we are returning contents here, it is not destructed (Dropped)
  Some(contents)
}
