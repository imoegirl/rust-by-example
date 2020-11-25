#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

pub fn run(){
    println!("p020_match_destructuring_enum >>>>>>>>");
    let color = Color::RGB(122, 17, 40);

    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("r: {}, g: {}, b: {}", r,g,b),
        Color::HSV(h, s, v) => println!("h: {}, s: {}, v: {}", h,s,v),
        Color::HSL(h, s, l) => println!("h: {}, s: {}, l: {}", h,s,l),
        Color::CMY(c, m, y) => println!("c: {}, m: {}, y: {}", c,m,y),
        Color::CMYK(c, m, y, k) => println!("c: {}, m: {}, y: {}, k:{}", c,m,y,k)
    }

    println!("\n");
}