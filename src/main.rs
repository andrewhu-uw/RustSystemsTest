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
  
  // Create the word HT
  let mut wordmap : HashMap<String, WordPositions> = HashMap::new();
  
  let args : Vec<String> = env::args().collect();
  if args.len() < 2 {
    println!("Enter a filename to read\nUsage: rs333 [FILE]");
    process::exit(1);
  }
  let filename : &String = &args[1];

  do_work(&mut wordmap, filename);

  for (word, wp) in wordmap {
    println!("{} : {}", word, wp.num());
  }
}

fn do_work(wordmap : &mut HashMap<String, WordPositions>, filename: &String) {
  // Collect the contents, quit if it's not ASCII
  let contents : String = match read_file(&filename) {
    None => { println!("File is not ASCII, exiting");
              return;
            },
    Some(x) => x,
  };
  println!("Contents of {} is:\n{}", filename, contents);
  
  // `contents` is moved _into_ the function
  loop_and_insert(wordmap, contents);
  // `contents` was already dropped at the end of loop_and_insert
  // wordmap is not dropped because it is a reference
}

// Reads `contents` and for each word, places a new position into the 
// WordPositions for at the key corresponding to that word

// What our implementation wants to do is move all of the str contents of 
// contents:String into the HashMap as keys, Unfortunately, we have to copy
// the entire contents, once, but only once to create new encapsulated String
// keys
fn loop_and_insert(map : &mut HashMap<String, WordPositions>, 
                    contents : String) {
  // Create an iterator over the words
  let split: std::str::SplitWhitespace =  contents.split_whitespace();

  // Look at each alphabetic word, and put its position in the corresponding
  // value
  for token in split {
    // Empty Strings are already filtered out by split_whitespace()
    //if token.len() == 0 { continue; }
    
    // Avoid get None's by inserting a new WP
    if !map.contains_key(token) {
      map.insert(token.to_owned(), WordPositions::new());
    }
    // Add one to the number of instances of this word found
    match map.get_mut(token) {
      Some(wp) => { wp.inc();
                                // TODO: Create a PositionIterator that splits 
                                // on whitespace
                    wp.add(3);  // FIXME: append the actual position
                  },
      None => panic!("map should have already had a value"),
    };
  }
  // `contents` is dropped here
}

fn read_file(pathstr : &str) -> Option<String> {
  let path : &Path = Path::new(pathstr);
  
  let mut file = match File::open(&path) {
    Err(why) => panic!("Couldn't open {}: {}", path.display(),
                  why.description()),
    Ok(_file) => _file,
  };
  
  // contents should be a String because we want a growable space that can be
  // moved 
  let mut contents = String::new();
  match file.read_to_string(&mut contents) {
    Err(why) => panic!("Couldn't open {}: {}", path.display(),
                  why.description()),
    Ok(_) => {},
  }
  
  if !contents.is_ascii() {
    return None
  }
  // `contents` is not dropped here because it was moved into Some, which was
  // then returned
  Some(contents)
}
