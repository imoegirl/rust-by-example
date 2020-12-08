pub fn run() {
    println!("p028_closure_capture >>>>>>>>");
    use std::mem;
    let color = "green";

    // 捕获了 color 的引用 &
    let print = || println!("`color`: {}", color);
    print();
    print();

    let mut count = 0;
    // 捕获了 count 的 &mut
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();
    inc();

    let movable = Box::new(3);
    // movable 的值被捕获，移动进了闭包中，此闭包只能调用一次
    let consume = || {
        println!("`moveable`: {:?}", movable);
        mem::drop(movable);
    };

    consume();

    let haystack = vec![1, 2, 3];
    // 强制获取被捕获变量的所有权
    let contains = move |needle| haystack.contains(needle);

    println!("contains 1: {}", contains(&1));
    println!("contains 4: {}", contains(&4));
    println!("\n");
}
