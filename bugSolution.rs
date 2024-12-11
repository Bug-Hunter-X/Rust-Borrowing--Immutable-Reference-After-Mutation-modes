fn main() {
    let mut x = 5;
    let y = &mut x; 
    *y = 10; 
    let z = &x; // Note: z is an immutable reference to x AFTER the mutation; it does not prevent the mutation.
    println!("{}", z); // Prints 10
    // The following would result in a compile-time error because y is still borrowed mutably
    //println!("{}", y);
}
