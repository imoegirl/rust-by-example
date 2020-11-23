fn loop1() {
    let mut count = 0u32;
    println!("Let's count until 10");
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }

        if count >= 10 {
            println!("break loop 1");
            break;
        }
    }
}

#[allow(unused_labels, unreachable_code)] // 这个是忽略警告的
fn loop_label() {
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Enterted the inner loop");
            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");
}

fn return_from_loop() {
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };

    println!("Count from loop: {}", result);
}

pub fn run() {
    println!("p015_loop >>>>>>>>");
    loop1();
    loop_label();
    return_from_loop();
    println!("\n");
}
