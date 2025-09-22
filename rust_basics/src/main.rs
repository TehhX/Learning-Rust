// Unused variables etc. won't create warnings.
#![allow(unused)]

// Anatomy of a function: fn <name>(<parameter>: <type>, ...) -> <ret type> {...}
fn min(a: i32, b: i32) -> i32 {
    // Note: Rust does not have a ternary operator. This is its equivalent:
    return (if (a < b) { a } else { b });
}

fn main() -> () {
// Basic types
    // Let will assign a value to a variable.
    let a = 10;       // Integer
    let b = "Hello!"; // String

    // The "mut" keyword will make a variable mutable. All variables are immutable by default.
    let mut c = 'A'; // Character
    c = 'B';

    // Specifiers like f32, f64, i32 etc. can be used to specify types. Use a ':' between ident and type.
    let d: f32 = 0.25f32; // Float
    let e: f64 = 0.75f64; // Double

    let f = false; // Boolean

// Functions
    let g: i32 = 5;
    let h: i32 = 10;
    let i: i32 = min(g, h);
}
