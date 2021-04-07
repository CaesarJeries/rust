use std::fmt;

struct Point3D {
    x: f32,
    y: f32,
    z: f32
}

impl fmt::Display for Point3D {
    // This is the exact function signature as defined by the fmt module
    // &self =  a reference to the printed object (a Point3D object)
    // f = a reference to a formatter. Think of this as an output stream
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Note: note semicolon. This will return the value of the last expression,
        // which is the return value of the writeln! macro (an fmt::Result)
        // adding a semicolon will changes this line from an expression to a statement, which evaluates to `()` (the unit type)
        writeln!(f, "[{:.7}, {:.7}, {:.7}]", self.x, self.y, self.z) // floating-point formatting: the .7 specifies to print 7 decimal points

    }
}


fn main() {
    let origin = Point3D {x: 0., y: 0., z:0.};

    println!("{}", origin); // using {} invokes the fmt::Display, whereas using {:?} invokes fmt::Debug
}
