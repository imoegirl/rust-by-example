pub fn run() {
    println!("p014 >>>>>>>>");
    let n = 5;
    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }
    let big_n = if n < 10 && n > -10 { 10 * n } else { n / 2 };
    println!("big_n: {}", big_n);
    println!("\n");
}
