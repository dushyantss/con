use std::io::{self, BufRead};

fn main() {
    for line in io::stdin().lock().lines() {
        match line {
            Err(_) => break,
            Ok(s) => println!("{}", s),
        }
    }
}
