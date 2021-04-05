// Source: https://doc.rust-lang.org/rust-by-example/hello/print.html


fn main() {

    let s1 = format!("{x} + {y} = {z}", x=1, y=2, z=3);
    let s2 = format!("{0} - {1} - {0}", "Hello", "Sup"); // notice how `sup` is passed twice

    println!("{}", s1);
    println!("{}", s2);

    // type specification
    println!("{}", 31); // default is i32
    println!("{}", 31i64); // i64 == long
    println!("{}", 31f32); // float
    println!("{}", 31f64); // double

    // special formatting
    // Special formatting can be specified after a `:`.
    // The only appropriate formatting traits are:
    //      - ``, which uses the `Display` trait
    //      - `?`, which uses the `Debug` trait
    //      - `e`, which uses the `LowerExp` trait
    //      - `E`, which uses the `UpperExp` trait
    //      - `o`, which uses the `Octal` trait
    //      - `p`, which uses the `Pointer` trait
    //      - `b`, which uses the `Binary` trait
    //      - `x`, which uses the `LowerHex` trait
    //      - `X`, which uses the `UpperHex` trait
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    println!("{0} in octa: {0:o}", 12); // Octal
    println!("{0} in hexa: {0:x}", 12); // Hexadecimal

    // Controlling the width of the output
    println!("{n:w$}|", n=3, w=7); // right-aligned (6 spaces and a `3`)
    println!("{n:>7}|", n=3); // right-aligned (6 spaces and a `3`)
    println!("{n:<7}|", n=3); // left-aligned (6 spaces and a `3`)
    println!("{n:<07}|", n=3); // left-aligned (6 zeros and a `3`)

    // Debugging
    println!("This format is used for display: {}", "What's changed?");
    println!("This format is used for debugging: {:?}", "What's changed?");
    println!("This format is used for debugging: {:?}", 3.14);
}
