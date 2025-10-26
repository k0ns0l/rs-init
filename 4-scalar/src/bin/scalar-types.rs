#![allow(unused)]
// #[deny(arithmetic_overflow)]

fn main() {
    // scalar types
    //  - single value
    //  - building blocks for more complex types


    //  signed-int
    //      tweak: 
    //          let i: isize = 1;   // arch deps
    

    //  unsigned-inti
    //      u[8, 16, 32, 64, 128]


    //  floats
    //      f[32, 64]


    //  bool
    //      bool[true, false]


    //  chars
    //      char
    



    // Type-conversion
    let i: i32 = 1;
    let j: u32 = i as u32;
    let k: u32 = j + (i as u32);

    println!("{0}\n{1}\n{2}", i, j, k);

    let min_i: i32 = i32::MIN;
    let max_i: i32 = i32::MAX;

    println!("i32 min: {0}\ni32 max: {1}", min_i, max_i);

    // [over, under]flows
    let mut u: u32 = u32::MAX;
    let mut v: u32 = u32::MAX;

    // u += 1;
    // println!("Overflow u32 - {u}");

    //  -------|    overflows CHECK with `checked_add` & `wrapping_add`
    let u = u32::checked_add(u32::MAX, 1);
    println!("SAFE `checked_add` overflow calls - \t{:?}", u);
    let v = u32::wrapping_add(u32::MAX, 1);
    println!("SAFE `wrapping_add` overflow calls - \t{v}");

}
