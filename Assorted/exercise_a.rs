struct Person {
    name: String,
    age: i32,
    fav_color: String
}

impl Person {
    fn print(&self) {
        println!("{}\n  Age: {}\n  Favourite Colour: {}",
            self.name,
            self.age,
            self.fav_color
        )
    }
}

fn main() {
    let vec = vec![
        Person {
            name:      "Timmy".to_owned(),
            age:       10,
            fav_color: "Yellow".to_owned()
        },
        Person {
            name:      "Jackson".to_owned(),
            age:       15,
            fav_color: "Amber".to_owned()
        },
        Person {
            name:      "Helena".to_owned(),
            age:       4,
            fav_color: "Red".to_owned()
        }
    ];

    for person in vec {
        if person.age <= 10 {
            person.print();
        }
    }
}
