extern crate rand;
use rand::{Rng, thread_rng};

fn main() {
    let mut x: i64 = 0;
    loop {
        if roll() {
            println!("You won! in {} moves", x);
            return;
        }
        x = x + 1;
    }
}

fn roll() -> bool {
    let x1 = myrnd();
    let x2 = myrnd();
    let x3 = myrnd();
    let x4 = myrnd();
    let x5 = myrnd();

    println!("{}{}{}{}{}", x1, x2, x3, x4, x5);
    if x1 == x2 && x2 == x3 && x3 == x4 && x4 == x5 {
        println!("Found a single roll yatzee! {}'s", x1);
        return true;
    }
    else {
        return false;
    }
}

fn myrnd() -> u32 {
    let mut rng = thread_rng();
    let x: u32 = rng.gen_range(1, 6);
    return x;
}
