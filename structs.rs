// Classic C struct
#[derive(Debug)]
struct Point3D {
    x: f32,
    y: f32,
    z: f32
}

// Tuple struct. Basically a named tuple
struct Pair(i32, i32);

// A unit struct
// field-less
// useful for generics
struct Unit;


fn main() {

    /*
    initialization of C structs is similar to C syntax
    Notes:
        - types must match. There are no implicit typecasts
        - all fields must be initialized
        - order is not important since we're explicitly specifying the field using
          its name
        - there's a shorthand syntax available: pass variables that have the same name
          as the data fields of the struct to the object creation expression.
    */

    /*
    Additional Notes:
    - #[derive(Display)] does not work for structs. From what I could find in the 
      docs it works for simple enum types.
    */
    
    let origin = Point3D {x:0., y:0., z:0.};

    // this will initialize the fields correctly
    let p1 = Point3D {z:3., x: 2., y:13.};
    
    let x = 0.;
    let y = 0.;
    let z = 0.;

    // shorthand
    let p2 = Point3D {x, y, z};

    println!("Origin: {:?}", origin);
    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);
}
