enum Color {
    Red,
    Green,
    Blue
}

// Ownership is moved to this function on call
fn display(c: Color) -> () {
    println!("{}",
        match c {
            Color::Red   => "Red",
            Color::Blue  => "Blue",
            Color::Green => "Green"
        }
    );
}

// This fn borrows the variable, doesn't move ownership
fn display_borrow(c: &Color) -> () {
    println!("{}",
        match c {
            Color::Red   => "Red",
            Color::Blue  => "Blue",
            Color::Green => "Green"
        }
    );
}

fn main() -> () {
    // Ownership for x, y, z lies in fn main
    let (x, y, _z) = (Color::Red, Color::Green, Color::Blue);

    // Ownership is moved to fn by move
    display(x);

    // Won't compile, ownership was already moved in above fn calls.
    // display(x);

    // Simply borrows ownership
    display_borrow(&y);
    // May be called again
    display_borrow(&y);
    // May be moved, as at this point ownership still resides in fn main
    display(y);
    // Because ownership was moved, cannot move again or borrow from this point (Following won't compile)
    // display_borrow(&y);
    // display(y);
}
