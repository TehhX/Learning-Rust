// Anatomy of a function:
// fn <name>(<parameter>: <type>, ...) -> <ret type> {...}
fn min(a: i32, b: i32) -> i32 {
    // Note: Rust does not have a ternary operator. This is its equivalent:
    return if a < b { a } else { b };
}

fn main() -> () {
    let x: i32 = 5;
    let y: i32 = 10;
    let z: i32 = min(x, y);
    println!("The min of {x} and {y} is {z}.");
}
