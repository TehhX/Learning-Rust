// Let will assign a value to a variable.
fn main() -> () {
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
}
