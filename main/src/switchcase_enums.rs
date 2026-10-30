enum Grade {
    A,
    B,
    C,
    F,
}

fn main() {
    let score = Grade::C;
    match score {
        Grade::A => println!("A"),
        Grade::B => println!("B"),
        Grade::C => println!("C"),
        Grade::F => println!("F"),
    }
}