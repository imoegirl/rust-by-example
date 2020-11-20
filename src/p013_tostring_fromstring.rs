// 将某种类型转换成字符串，可以实现ToString trait，
// 但更好的选择是实现实现 fmt::Display trait

// 而从字符串转为目标类型，只要目标类型实现了 FromStr trait
// 就可以使用 "value".parse::<目标类型>().unwrap() 来转换

use std::str::FromStr;
use std::string::ToString;

use std::num::ParseIntError;

#[derive(Debug)]
struct Circle {
    radius: i32,
}

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius: {:?}", self.radius)
    }
}

impl FromStr for Circle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let radius = s.parse::<i32>()?;
        Ok(Self { radius })
    }
}

pub fn run() {
    println!("p013_tostring_fromstring >>>>>>>>");
    let circle = Circle { radius: 10 };
    println!("Circle: {:?}", circle);

    let s_radius = "12";
    let circle = s_radius.parse::<Circle>().unwrap();
    println!("Circle from str: {:?}", circle);
    println!("\n");
}
