#![allow(unused)]

// auto-generate Debug attr
#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
    Rgba(u8, u8, u8, f32),
    Hex(String),
    Biv {b: u8, i: u8, v: f64},
}


fn main() {
    // enum - finite state value
    let color_red = Color::Red;
    let color_green = Color::Green;

    let color: Color = Color::Rgba(255, 255, 255, 0.1);
    let color2 = Color::Hex("#ffffff".to_string());
    let color3 = Color::Biv { b:0, i:0, v:3.143 };
    

    // attributes - Debug and PartialEq
    println!("Compare enums (red VS green) => {}", color_red == color_green);
    println!("Auto-generate Debug {:?}", color3);

    // Option = Some(-1) | None
    let x: Option<i32> = None;
    let x: Option<i32> = Some(-1);

    // Result = Ok(5) | Err(false)
    let res: Result<i32, String> = Ok(5);
    let res: Result<i32, bool> = Err(false);
    println!("Result = {:?}", res);
}
