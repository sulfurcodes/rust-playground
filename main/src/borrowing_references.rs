fn main() {
    let name = String::from("sulfur");

    let len = calculate_length(&name); //passing the reference of name, passing just name would give the ownership to the function

    println!("Name: {}", name);
    println!("Length: {}", len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// pass by value vs pass by reference icl