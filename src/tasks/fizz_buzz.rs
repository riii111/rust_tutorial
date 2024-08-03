pub fn run() {
    let res = (1..=100).fold(format!(""), |buf, x| match (x % 3, x % 5) {
        (0, 0) => format!("{}FizzBuzz\n", buf),
        (0, _) => format!("{}Fizz\n", buf),
        (_, 0) => format!("{}Buzz\n", buf),
        _ => format!("{}{}\n", buf, x),
    });
    println!("{}", res);
}
