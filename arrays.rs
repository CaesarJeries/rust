
// note: prefixing variable names with an underscore
// suppresses warning for unused variables
fn main() {
    let _a1 = [1, 2, 3];

    // array type annotation
    // the number of elements must match the number listed
    // in the type annotation
    let _a2: [i8; 3] = [1, 2, 3];

    // initialize an array of size 5 filled with 3's
    let _a3 = [3; 5];

    let a4: [i8; 13]; // creates an uninitialized array. This actually compiles
    //println!("a4[0]: {}", a4[0]); // but if you try to access an element, you'll
                                    // get an error[E0381]: use of possibly-uninitialized variable
}
