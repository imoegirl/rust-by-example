struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

fn red<T: Red>(_: &T) -> &'static str {
    "red"
}

fn blue<T: Blue>(_: &T) -> &'static str {
    "blue"
}

pub fn run() {
    println!("p043_empty_bound >>>>>>>>");
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    // println!("A turkey is {}", red(&_turkey)); // 报错，不符合约束
    println!("\n");
}
