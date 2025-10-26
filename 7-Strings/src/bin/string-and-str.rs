#![allow(unused)]

fn main() {
    // String = vector of u8 (Vec<u8>) valid UTF-8
    // &str = slice of u8 (&[u8]) valid UTF-8
    
    let msg: String = String::from("Hello Rust 🦀");
    // let msg_ref: &str = &msg;
    let len: usize = msg.len();
    // println!("msg: {msg}\nlen: {len}\n{msg_ref}");

    
    // String slice -
    let s: &str = &msg[0..5];

    
    // binary hardcoded - IMMUTABLE!
    let hello: &str = "Hello Rust 🦀";

    
    // multi line STDOUT
    let multi_line: &str = r#"
        {"name": "john",
        "age": 12,
        "gender": 'm'}
    "#;
    // println!("blob = {:?}", multi_line);

    
    // Adding &str to String func
    let mut msg: String = "Hello Rust".to_string();
    msg += "!";
    println!("{msg}");


    let emoji = "🦀🦀";
    let greetings = format!("STDOUT:: {} {}", emoji, msg);
    println!("{greetings}");
}
