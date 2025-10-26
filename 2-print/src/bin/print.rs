#![allow(unused)]

#[derive(Debug)]
struct Lang {
    language: String,
    version: String,
}

fn main() {
    let lang = "rust";
    println!("{lang}");

    let lang = Lang {
        language: "rust".to_string(),
        version: "1.9.1".to_string(),
    };

    let x = 2;
    println!("{:#?}", lang);

    println!("{0} x {0} = {1}", x, x * x);
}
