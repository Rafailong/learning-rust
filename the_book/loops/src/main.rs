use rand::{Rng, thread_rng};

fn main() {
    // loop is kinda infinite.
    // we can `break` and `continue`
    let mut counter: u8 = 1;

    loop {
        println!("again! - count = {}", counter);

        counter += 1;

        if counter <= 10 {
            continue;
        } else {
            break;
        }
    }

    let mut n = thread_rng().gen_range(1..101);
    while n > 0 {
        println!("n = {}", n);
        n -= 1;
    }
    println!("LIFT-OFF!");


    let arr = [1, 2, 3, 4, 5];
    for number in arr {
        println!("number = {}", number);
    }

    for i in 0..5 {
        println!("{}!", i);
    }
    println!("LIFT-OFF!");
}
