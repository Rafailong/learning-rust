fn another_function() {
    println!("another function.");
}

fn print_number(n: i32) {
    println!("number is: {}", n);
}

fn print_labeled_number(n: i32, label: char) {
    println!("value: {} - label: {}", n, label);
}

fn last_exp_is_return_by_default() -> u32 {
    // your beautiful code here
    5
}

fn early_return(n: u32) -> bool {
    if n < 50 { return true }
    false
}

fn main() {
    println!("Hello, world!");

    another_function();

    print_number(10);

    // calling a function is an expression too
    print_labeled_number(1, 'm');

    let x = 5; // this is an statement

    let z = { // a code-block is an expression
        let y = 15;
        // tricky bit: no `;` so the expression value is returned.
        // NOTE: expression ending with `;` are statements
        x + y
    };
    println!("z = {}", z);

    println!("early return? {}", early_return(10));
    println!("early return? {}", early_return(60));
}
