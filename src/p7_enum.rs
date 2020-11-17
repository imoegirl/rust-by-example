enum WebEvent {
    PageLoad,
    PageUnLoad,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnLoad => println!("page unload"),
        WebEvent::KeyPress(value) => println!("Key pressed: {}", value),
        WebEvent::Paste(value) => println!("Paste {}", value),
        WebEvent::Click { x, y } => println!("Clicked at x: {} y: {}", x, y),
    }
}

#[allow(dead_code)]
enum VeryVerboseEnumOfthingsToDoWithNumbers {
    Add,
    Subtract,
}

// 使用别名
type Operations = VeryVerboseEnumOfthingsToDoWithNumbers;

#[allow(dead_code)]
// Self 代表当前实现的类型
impl VeryVerboseEnumOfthingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

pub fn run() {
    println!("p7_enum >>>>>>>>");
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnLoad;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let _opt = Operations::Add;

    println!("\n");
}
