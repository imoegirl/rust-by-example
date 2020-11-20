#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

pub fn run() {
    println!("p001_debug >>>>>>>>");
    let name = "Peter";
    let age = 22;
    let peter = Person { name, age };
    println!("{:?}", peter);
    println!("\n");
}
