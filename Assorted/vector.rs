fn main() -> () {
    // Use vec! macro.
    let vals = vec![3, 6, 9, 12, 15];

    // [n] is slice notation.
    println!("{} {}", vals[0], vals[1]);

    // Create a new vector:
    let mut new_vec: Vec<String> = Vec::new();
    new_vec.push("Wow".to_owned());
    new_vec.push("True".to_owned());
    new_vec.push("Forever".to_owned());
    new_vec.push("Yeah".to_owned());

    // Note borrow, without would mean a move into the loop.
    for str in &new_vec {
        println!("{str}");
    }
}
