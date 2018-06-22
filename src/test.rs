#![allow(unused)]
fn main() {
let copy:String;
let v;
let sp:String;
v = {
    let s = "hello world".to_string(); // String type implements Clone

    let vtmp : Vec<&str>= s.split_whitespace().collect::<Vec<&str>>();
    let mut vq = Vec::new();
    for token in vtmp {
        // Cannot do token.clone(), because even though it creates separate data,
        // that data will be dropped at the end of the scope block because it was not
        // _moved_ out of this scope block
        vq.push(token.to_owned());
    }
    vq
};
//println!("{}",sp);
println!("{:?}",v);


}

//fn foo() -> String