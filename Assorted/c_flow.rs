fn main() -> () {
    // Simple if/else-if/else blocks:
    if 1 == 1 {
        println!("True.");
    }
    else if 1 == 3 {
        println!("False?");
    }
    else {
        println!("False!");
    }

    // Ternary-like (with notably more than 3 parts) statement for the same effect:
    println!("{}", if 1 == 1 { "True." } else if 1 == 3 { "False?" } else { "False!" });

    // Match statement (Similar to switch statements in other languages):
    let x: i32 = 16;
    match x {
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
    let _y = match x {
        16 => 10,
        _  => 15,
    };
}
