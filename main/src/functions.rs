fn main() {
    greet("sulfur");
    let sum1 = add1(2,3);
    let sum2 = add2(2,3);
    println!("sum1 is {}", sum1);
    println!("sum2 is {}", sum2);
}

// normal function
fn greet(name: &str) {
    println!("Hello {}", name);
}

// function returning a fixed value of type 32 bit integer 
fn add1(a: i32, b: i32) -> i32 {
    return a + b; 
}

// function returning a fixed value of type 32 bit integer 
fn add2(a: i32, b: i32) -> i32 {
    a + b
}

// both way or returning works somehow