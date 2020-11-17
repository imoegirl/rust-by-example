type NanoSecond = u64;
type Inch = u64;

#[allow(non_camel_case_types)]
type u64_t = u64;

pub fn run() {
    println!("p10_aliasing >>>>>>>>");
    let nanoseconds: NanoSecond = 6 as u64_t;
    let inches: Inch = 2 as u64_t;
    println!("nanoseconds: {} inches: {}", nanoseconds, inches);
    println!("\n");
}
