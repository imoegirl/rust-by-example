/*
    Fn, FnMut, FnOnce，相当于权力越来越大。编译器在编译时，都以尽可能权力小的方式捕获变量。
    apply 的 trait 约束虽然是 FnOnce，但是传入的闭包如果是 Fn 或 FnMut 也是可以的。
    但是反过来，如果 trait 约束是 Fn，这时传入 FnMut 或 FnOnce 形式的闭包，将编译出错。
    也就是说，Fn FnMut FnOnce 类似一种包含关系。
*/

fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

pub fn run() {
    println!("p029_closure_as_input >>>>>>>>");
    use std::mem;
    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        // 走到下面这一句，diary 是以&的形式捕获变量，Fn
        println!("I said {}.", greeting);

        // 走到这里，因为改为难了，farewell，所以需要以&mut的形式
        // 也就变为了 FnMut
        farewell.push_str("!!!");
        println!("Then i screamed {}.", farewell);
        println!("Now I can sleep. zzzzzzz");

        // 这里手动调用了drop，需要通过值获取 farewell
        // 现在闭包需要是 FnOnce
        mem::drop(farewell);

        // 最终，这个闭包捕获变量的形式是 FnOnce
    };

    apply(diary);

    let double = |x| 2 * x;
    // 闭包 double 满足 apply_to_3 的 trait 约束
    println!("3 doubled: {}", apply_to_3(double));

    println!("\n");
}
