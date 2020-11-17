// 下面定义的常量在当前文件都可用，如果前面加上pub，则在其他文件中也可以使用
static LANGUAGE: &str = "Rust"; // 静态变量，在unsafe 中是可以改变的
const THRESHOLD: i32 = 10; // 常量

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

pub fn run() {
    println!("p9_constants >>>>>>>>");
    let n = 16;
    println!("this is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
    println!("\n");
}
