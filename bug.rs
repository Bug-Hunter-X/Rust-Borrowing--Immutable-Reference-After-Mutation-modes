fn main() {
    let mut x = 5;
    let y = &mut x; 
    *y = 10; 
    let z = &x; // this is allowed because the value of x is immutable
    println!("{}", z); 
}