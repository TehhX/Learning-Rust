struct Book {
    id: u64,
    rel_year: i32
}

// Implementation block
impl Book {
    // Self can be replaced by Book, they mean the same thing. Self will automatically change to whatever the name of the obj is.
    fn none() -> Self {
        return Self { id: 0, rel_year: 0 };
    }

    // Takes a reference (borrow) to self, as a method would.
    fn display(&self) -> () {
        println!("ID: {}\nYR: {}\n", self.id, self.rel_year);
    }
}

fn main() -> () {
    let b = Book {
        id: 0,
        rel_year: 2010
    };
    b.display();

    let n = Book::none();
    n.display();
}
