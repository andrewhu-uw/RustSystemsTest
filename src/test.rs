#![allow(unused)]
fn main() {
let copy:String;
let v: Vec<String>;
let sp:String;
v = {
    let s = "hello world".to_string(); // String type implements Clone

    let vtmp : Vec<&str>= s.split_whitespace().collect::<Vec<&str>>();
    let mut vq = Vec::new();
    for token in vtmp {
        vq.push(token.to_owned());
    }
    vq
};
//println!("{}",sp);
println!("{:?}",v);


}

//fn foo() -> String