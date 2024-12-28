#![allow(unused)]

use std::vec;

// help: the trait `Debug` is not implemented for `Lang` -- LOC: 29
// add `#[derive(Debug)]` to `Lang` or manually `impl Debug for Lang`
#[derive(Debug)]

struct Lang {
    language: String,
    version: String,
}

fn main() {
    let lang = "world";
    println!("Hello, {}!", lang);

    let lang = "Rust";
    println!("Hello, {lang}!");

    // Integer operation
    let x = 5;
    println!("{} x {} = {}", x, x, x * x);

    // LESSON 2: Accessing struct element
    // Linting using ``cargo fmt``
    // -------------------------------------------
    let lang = Lang {
        language: "rust".to_string(),
        version: "1.77.1".to_string(),
    };

    println!("{:?}", lang);

    // LESSON 3: Variables
    // -------------------------------------------
    let mut y: u32 = 4;
    y += 1;
    println!("y = {}", y);

    const PI: f64 = 3.14159;
    let z: bool = false;

    // Infer type
    let vec: Vec<_> = vec![1, 2, 3];
    println!("{:#?}", vec);


}
