pub fn run() {
    println!("p016_while >>>>>>>>");
    let mut n = 1;
    while n < 10 {
        if n % 2 == 0 {
            println!(" {} % 2 == 0", n);
        } else if n % 3 == 0 {
            println!(" {} % 3 == 0", n);
        } else {
            println!("now n is {}", n);
        }
        n += 1;
    }
    println!("\n");
}
