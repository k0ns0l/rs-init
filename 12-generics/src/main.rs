// Use generics to create def for items like function signatures.
// The identity function accepts a value of type T and returns it unchanged.
fn identity<T>(value: T) -> T {
    value
}

// !DEBUG PURPOSE ONLY
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    let x = identity(5);        // x is inferred to be i32
    let y = identity(3.14);      // y is inferred to be f64
    let z = identity("Hello");   // z is inferred to be &str

    print_type_of(&x);
    print_type_of(&y);
    print_type_of(&z);
}
