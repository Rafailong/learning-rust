
fn main() {
    const MY_PI: f64 = 3.1416;

    // immutable
    let y: u32 = 123;

    // mutable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // shadowing
    // from here on, y is a String not a u32
    let y = String::from("y");

    // arrays as zero-based indexed
    let arr: [u8; 5] = [1, 2, 3, 4, 5];
    arr[0]; // 1
    // arr[5]; does no compile because out-of-index
}
