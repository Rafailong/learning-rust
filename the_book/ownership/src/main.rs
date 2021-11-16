mod return_values_and_scope;
mod slices;

fn main() {

    let s = String::from("hello");  // s comes into scope

    // s's value moves into the function...and so is no longer valid here
    takes_ownership(s);

    // println!("{}", s); // compile error

    let x = 5; // x comes into scope

    // x would move into the function,
    // but i32 is Copy, so it's okay to still
    // use x afterward
    makes_copy(x);

    println!("x = {}", x);
} // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.