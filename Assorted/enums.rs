// Allows unused enum values:
#![allow(dead_code)]

fn main() -> () {
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
    match x {
        Color::Red     => println!("Red"),
        Color::Blue    => println!("Blue"),
        Color::Green   => println!("Green"),
        Color::Cyan    => println!("Cyan"),
        Color::Magenta => println!("Magenta"),
        Color::Yellow  => println!("Yellow"),
        Color::Orange  => println!("Orange"),
    }
}
