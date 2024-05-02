use std::env;
use english_numbers::*;

fn main() {
for argument in env::args() {
    println!("{argument}");
}
    let count: i64 = env::args().skip(1).next().map(|s| s.parse().expect("Max number")).unwrap_or(1_000_000);
    print!("{}", convert_all_fmt(0));
    for i in 1..=count {
        print!(", {}", convert_all_fmt(i));
    }
    println!(".");
}
