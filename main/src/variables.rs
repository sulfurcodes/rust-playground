fn main() {
    let x:i32 = 5; // immutable 32 bit integer
    let mut y: i32 = 5; // mutable 32 bit integer
    const Z: u32 = 50; // constant non negative unsigned 32 bit integer
    println!("x is {}", x); // {} are placeholders for printing varigables 
    println!("y is {}", y); 
    y = y + 1;
    println!("y is {}", y);
    println!("Z is {}", Z);
}

// cargo run