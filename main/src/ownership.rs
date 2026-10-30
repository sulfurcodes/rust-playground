fn main() {
    let s1: &str = "sulfur";
    let s2: &str = s1; // not s2 has the ownership of the string sulfur, s1 can no longer access it
    let s3: &str = s2.clone(); // clone() lets us copy the string without taking ownership away from source
    // println!("{}", s1); error
    println!("{}", s2);
    println!("{}", s3);
}