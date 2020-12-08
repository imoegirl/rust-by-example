// 即然闭包可以作为输入参数，那么满足 trait 约束的函数，也可以作为其参数

fn call_me<F: Fn()>(f: F) {
    f();
}

fn function() {
    println!("I'm a function!");
}

pub fn run() {
    println!("p030_function_as_input >>>>>>>>");
    let closure = || println!("I'm a closure!");
    call_me(closure);
    call_me(function);
    println!("\n");
}
