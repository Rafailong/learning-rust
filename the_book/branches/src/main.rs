use rand::{Rng, thread_rng};

fn main() {
    let n = thread_rng().gen_range(-100..101);
    println!("n = {}", n);

    if n == 0 {
        println!("n is zero");
    } else if n < 0 {
        println!("n is negative");
    } else {
        println!("en is positive");
    }

    // `if` is an expression so we can assign it result
    let even_or_odd = if n % 2 == 0 { "even" } else { "odd" };
    println!("n is {}", even_or_odd);
}
