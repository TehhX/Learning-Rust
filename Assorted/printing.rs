fn main() -> () {
    // Prints the contents, appends '\n'. The '!' denotes a macro, like with C++ and C:
    println!("Hello world!");

    /* The above line expands to:
    std::io::_print(std::format_args_nl!("Hello world!"));
    However, it contains unstable library feature(s), and produces an error, so it has been commented out. */

    // Print formatted just requires the variable name inside curly braces:
    let x: i32 = 15;
    println!("Data: {x}");

    // Different traits can be passed, such as binary print:
    println!("Other data: {x:b}");

    // Decimal places can be specified:
    println!("Other other data: {:10.10}", 1.234f32);
}
