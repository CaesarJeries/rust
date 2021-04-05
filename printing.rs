// Source: https://doc.rust-lang.org/rust-by-example/hello/print.html


fn main() {

    let s1 = format!("{x} + {y} = {z}", x=1, y=2, z=3);
    let s2 = format!("{0} - {1} - {0}", "Hello", "Sup"); // notice how `sup` is passed twice

    println!("{}", s1);
    println!("{}", s2);

    // type specification
    println!("{}", 31); // default is i32
    println!("{}", 31i64);
}