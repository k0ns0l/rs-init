#![allow(unused)]

fn main() {
    let mut arr: [u32; 10] = [0; 10];
    arr[2] = 10;

    // println!("{:?}", arr);

    // array slicing
    let nums: [i32; 5] = [-2, -1, 0, 1, 2];
    let s = &nums[0..3]; // first 3 elements ..
    println!("s = {:?}", s);
}
