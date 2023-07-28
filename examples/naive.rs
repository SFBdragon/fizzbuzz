
fn main() {
    for line in 1.. {
        if line % 15 == 0 {
            println!("FizzBuzz");
        } else if line % 5 == 0 {
            println!("Buzz");
        } else if line % 3 == 0 {
            println!("Fizz");
        } else {
            println!("{}", line);
        }
    }
}
