# Rust Borrowing: Immutable Reference After Mutation
This example demonstrates a scenario that might initially appear confusing regarding Rust's borrowing rules.  The key concept to understand is that the immutability of a reference is to the value at the time of borrowing, not the value before the borrowing. 

The original code creates a mutable reference `y` and mutates the value of `x`. Then, creates an immutable reference `z` to `x`.  This is perfectly valid in Rust, even though `x` was mutated.  The immutable reference `z` only 'sees' the final value of `x`. 

## How to Fix/Improve
The code is not incorrect but illustrates a subtle point in Rust's borrowing system. The solution is just to make clear the subtle nature of this concept using comments.