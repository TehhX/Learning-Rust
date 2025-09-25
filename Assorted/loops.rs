fn main() -> () {
    // Regular loop:
    let i: bool = true;
    loop {
        println!("Looping...");

        if i {
            break;
        }
    }

    // While loop:
    let mut i: u128 = 0;
    while i < 5 {
        println!("{} = {}", i + 1, i.pow(2u32));
        i += 1; // Pre/post-crement operators don't exist in Rust.
    }

    // For loop
    for i in 0..=5 {
        println!("{i}");
    }

    // See vector.rs for info on vectors.
    let vec = vec![3, 6, 9];
    for num in vec {
        print!("{num}, ");
    }
    println!();
}
