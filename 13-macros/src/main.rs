macro_rules! say_hello {
    // `expr` covers both `ident` & `literal`
    () => {
        // The macro will expand into the contents of this block.
        println!("Hello!")
    };
}

fn main() {
    say_hello!();
}
