#![allow(unused)]

fn main() {
    let t: (bool, u32, char) = (true, 1, 'c');

    // De-structure
    let (a, b, c) = t;

    // Ignore with `_`
    let (_, b, _) = t;

    // empty tuple - unit tuple
    let t = ();

    // nested tuple
    let nested = ((1.23, 'a'), (true, 1, 'c'), ());
    let t: (bool, u32, char) = (true, 1, 'c');

    // println!("t = {} {} {}", t.0, t.1, t.2);
    println!("nested = {} {:#?}", nested.0.0, nested.1);
}
