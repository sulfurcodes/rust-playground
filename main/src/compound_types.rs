fn main() {
    // tuple
    let person: (&str, u32, u32); //a tuple declared with fixed data types
    person = ("sulfur", 19, 67000);
    println!("{} is a {} years old with a debt of {}", person.0, person.1, person.2);

    // array
    let people: [&str; 3]; //a variable declared with string datatype of size 3
    people = ["sulfurcodes", "sulfurores", "pirate"];
    println!("the names are {}, {}, and {}", people[0], people[1], people[2])
}

// compound data types 