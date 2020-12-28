fn give_princess(gift: &str) {
    if gift == "snake" {
        panic!("AAAaaaaa!!!");
    }
    println!("I love {}s !!!!!!", gift);
}

pub fn run() {
    println!("p058_panic >>>>>>>>");
    give_princess("teddy bear");
    //give_princess("snake"); // 这一句将造成主动 panic
    println!("\n");
}
