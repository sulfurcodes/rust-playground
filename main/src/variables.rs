fn main() {
    let x:i32 = 5; // immutable integer type of size 32 bits
    let mut y: i32 = 5; // mutable integer of size 32 bits
    const Z: u32 = 50; // constant unsigned variable of size 32 bits 
    println!("x is {}", x); // {} are placeholders for printing
    println!("y is {}", y); 
    y = y + 1;
    println!("y is {}", y);
    println!("Z is {}", Z);
}

// cargo run