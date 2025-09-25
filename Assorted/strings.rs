// &str is a borrowed string slice.
fn prints(s: &str) {
    println!("{s}");
}

fn printos(s: String) {
    // As an aside, using the debug token (?) will enclose the string in quot. marks.
    println!("{s:?}");
}

fn main() {
    // String is an owned string.
    let my_string: String = "Hello".to_owned();
    prints(&my_string);
    printos(my_string);

    // Won't compile, already moved to printos
    // prints(&my_string);
}
