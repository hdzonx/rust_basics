use std::io;
mod calculator;

fn main() {
    use crate::calculator::calculator::Calculator;

    println!("{:-^50}", "SUM VALUES CALCULATOR");
    let mut s = String::new();
    let banner = "\n\
    Write the sequences of number\n\
     separated by comma.\n\
      Example: 1,2,3, 4, 5\n\
      ";

    println!("{}", banner);
    io::stdin()
        .read_line(&mut s)
        .expect("Error handling console");

    Calculator::set_values(&s);
}
