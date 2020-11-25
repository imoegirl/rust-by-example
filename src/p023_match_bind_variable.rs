pub fn run(){
    println!("p023_match_bind_vairable >>>>>>>>");
    let age = 15;

    match age {
        0 => println!("I'm not born yet i guess"),

        // 如果age是在1~12之间，则将值绑定到n上，以便在分支中使用
        n @ 1 ..=12 => println!("I'm a child of age: {}", n),
        n @ 13 => println!("i'm a teen of age: {}", n),
        n => println!("I'm an old person of age: {}", n),
    }
    println!("\n");
}