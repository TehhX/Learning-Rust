// Allows unused member variables:
#![allow(dead_code)]

fn main() -> () {
    struct Shape {
        len_x: f32,
        len_y: f32,
        len_z: f32,
    }

    let rect_prism = Shape { len_x: 3.14, len_y: 9.81, len_z: 6.9 };

    let width = rect_prism.len_x;
    println!("That makes for a width of {width}m.");
}
