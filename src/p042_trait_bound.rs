use std::fmt::{Debug, Display};

#[allow(dead_code)]
fn printer<T: Display>(t: T) {
    println!("{}", t);
}

#[allow(dead_code)]
struct S<T: Display>(T);

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64,
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

pub fn run() {
    println!("p042_trait_bound >>>>>>>>");
    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };
    let _triangle = Triangle {
        length: 3.0,
        height: 4.0,
    };
    print_debug(&rectangle);
    println!("Area: {}", rectangle.area());

    // print_debug(&_triangle); // 会报错，因为未实现Debug
    println!("\n");
}
