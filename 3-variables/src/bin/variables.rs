#![allow(unused)]

fn main() {
    let x: i32 = -123;

    // println!("x is {x}");
    // Lets understand mutables and immutables
    let mut y: i32 = 123;
    y += 1;

    const NUM: u32 = 1;
    let z: bool = true;

    // vectors -- infering type!
    let v: Vec<_> = vec![1, 2, 3];
    
    println!("{0} {1:#?} {2}", x, y, z);
    println!("rs infers type: example - Vector `v` fancy printed\n{:#?}", v);
}
