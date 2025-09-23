// Unused variables etc. won't create warnings.
#![allow(unused)]

// Anatomy of a function:
// fn <name>(<parameter>: <type>, ...) -> <ret type> {...}
fn min(a: i32, b: i32) -> i32 {
    // Note: Rust does not have a ternary operator. This is its equivalent:
    return (if (a < b) { a } else { b });
}

fn main() -> () {
// ===== Basic types =====
    // Let will assign a value to a variable.
    {
        let x = 10;       // Integer
        let y = "Hello!"; // String
    }

    // The "mut" keyword will make a variable mutable. All variables are immutable by default.
    {
        let mut x = 'A'; // Character
        x = 'B';
        println!("{x}");
    }

    // Specifiers like f32, f64, i32 etc. can be used to specify types. Use a ':' between ident and type.
    {
        let x: f32 = 0.25f32; // Float
        let y: f64 = 0.75f64; // Double
    }

    // Bool type:
    {
        let x = false; // Boolean
    }

// ===== Functions =====
    {
        let x: i32 = 5;
        let y: i32 = 10;
        let z: i32 = min(x, y);
        println!("The min of {x} and {y} is {z}.");
    }

// ===== Printing =====
    // Prints the contents, appends '\n'. The '!' denotes a macro, like with C++ and C:
    println!("Hello world!");

    /* The above line expands to:
    std::io::_print(std::format_args_nl!("Hello world!"));
    However, it contains unstable library feature(s), and produces an error, so it has been commented out. */

    // Print formatted just requires the variable name inside curly braces:
    {
        let x: i32 = 15;
        println!("Data: {x}");

        // Different traits can be passed, such as binary print:
        println!("Other data: {x:b}");

        // Decimal places can be specified:
        println!("Other other data: {:10.10}", 1.234f32);
    }

// ===== Control flow =====
    // Simple if/else-if/else blocks:
    if (1 == 1) {
        println!("True.");
    }
    else if (1 == 3) {
        println!("False?");
    }
    else {
        println!("False!");
    }

    // Ternary-like (with notably more than 3 parts) statement for the same effect:
    println!("{}", (if (1 == 1) { "True." } else if (1 == 3) { "False?" } else { "False!" }));

    // Match statement (Similar to switch statements in other languages):
    {
        let x: i32 = 16;
        match (x) {
            10 => println!("It's 10."),
            15 => println!("It's 15."),
            15.. => println!("Between 16 and infinity."),
            0..=9 => println!("Between 0 and 9."),
            _ => println!("Unknown!"),
        }

        /* "<val>" means match a value specifically
           "<valA>..<valB>" means match some value between valA and valB EXCLUSIVELY
           "<valA>..=<valB>" means the same as above, this time inclusively
           "<val>.." means from val to infinity exclusively
           "..<val>" means from -infinity to val exclusively
           "_" means any unaccounted for value
           "default" is the same as "_" */

        // Match statement as expression (many things like c-flow statements are actually expressions in Rust):
        let y = match(x) {
            16 => 10,
            _  => 15,
        };
    }

// ===== Loops =====
    // Regular loop:
    let i: bool = true;
    loop {
        println!("Looping...");

        if (i) {
            break;
        }
    }

    // While loop:
    let mut i: u128 = 0;
    while (i < 5) {
        println!("{} = {}", i + 1, i.pow(2u32));
        i += 1;
    }

    // Enums:
    {
        // Note that things like enums, structs, funcs etc. may be nested in other funcs and subnested in curlies.
        enum Color {
            Red,
            Green,
            Blue,
            Cyan,
            Magenta,
            Yellow,
            Orange,
        }

        let x = Color::Red;
        match (x) {
            Color::Red     => println!("Red"),
            Color::Blue    => println!("Blue"),
            Color::Green   => println!("Green"),
            Color::Cyan    => println!("Cyan"),
            Color::Magenta => println!("Magenta"),
            Color::Yellow  => println!("Yellow"),
            Color::Orange  => println!("Orange"),
        }
    }

    // Structs:
    {
        struct Shape {
            len_x: f32,
            len_y: f32,
            len_z: f32,
        }

        let rect_prism = Shape { len_x: 3.14, len_y: 9.81, len_z: 6.9 };

        let width = rect_prism.len_x;
        println!("That makes for a width of {width}m.");
    }

    // Tuples store multiple values anonymously:
    {
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

        let tuple_one: (i32, i32) = (15, 30);

        let a = 3;
        let b = 9;

        let tuple_two = pair(a, b);

        separate_and_print(tuple_one);
        separate_and_print(tuple_two);

        // Example of struct and tuple usage:
        struct Object {
            pos: (f64, f64, f64), // Current position.
            vel: (f64, f64, f64), // Current velocity
            id: u128              // ID
        }

        let mut guy = Object { pos: (0.0, 0.0, 0.0), vel: (10.0, 5.0, 2.5), id: u128::MAX };

        println!("\nGuy movements over t:");
        let mut i = 0;
        while (i < 15) {
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
}
