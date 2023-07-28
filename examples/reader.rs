use std::io::BufRead;

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut lineno = 1;
    let mut buf = String::new();

    loop {
        stdin.read_line(&mut buf).unwrap();
        buf.truncate(buf.len() - 1);

        println!("{} {}", buf, buf.len());

        if lineno % 15 == 0 {
            assert!(buf == "FizzBuzz");
        } else if lineno % 5 == 0 {
            assert!(buf == "Buzz");
        } else if lineno % 3 == 0 {
            assert!(buf == "Fizz");
        } else {
            assert!(buf == lineno.to_string());
        };

        lineno += 1;
        buf.clear();
    }
}
