fn pair(x: i32, y: i32) -> (i32, i32) {
    return (x, y);
}

fn separate_and_print(input: (i32, i32)) -> () {
    // This creates two i32 vars x and y, assigning input.0 to x and input.1 to y.
    let (x, y) = input;
    /* It is functionally the same as:
    let x = input.0;
    let y = input.1; */

    println!("({x}, {y})");
}

// Example of struct and tuple usage:
struct Object {
    pos: (f64, f64, f64), // Current position.
    vel: (f64, f64, f64), // Current velocity
    id: u128              // ID
}

fn main() -> () {
    let tuple_one: (i32, i32) = (15, 30);

    let a = 3;
    let b = 9;
    let tuple_two = pair(a, b);

    separate_and_print(tuple_one);
    separate_and_print(tuple_two);

    let mut guy = Object { pos: (0.0, 0.0, 0.0), vel: (10.0, 5.0, 2.5), id: u128::MAX };

    println!("\nGuy movements over t:");
    let mut i = 0;
    while i < 15 {
        guy.pos.0 += guy.vel.0;
        guy.pos.1 += guy.vel.1;
        guy.pos.2 += guy.vel.2;

        println!("    T = {i:2}: ({:6.2}, {:5.2}, {:5.2}) at ({:5.2}, {:3.2}, {:3.2}) with ID {}.", guy.pos.0, guy.pos.1, guy.pos.2, guy.vel.0, guy.vel.1, guy.vel.2, guy.id);
        i += 1;
    }
    println!();

    // Tuples are also used for multiple-initialization let statements:
    let (one, two, three) = (1, 2, 3);
    println!("{one}, {two}, {three}.");

    // Different type tuples:
    let holy_tuple = ("Name", 123);
    println!("0: {}, 1: {}", holy_tuple.0, holy_tuple.1);
}
