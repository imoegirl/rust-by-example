fn give_commoner(gift: Option<&str>) {
    match gift {
        Some("snake") => println!("Yuck! I'm throwing that snake in a fire"),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No gift? oh well."),
    }
}

fn give_princess(gift: Option<&str>) {
    let inside = gift.unwrap(); // 这里如果是 None，就会直接 panic
    if inside == "snake" {
        panic!("AAAaaaaa!!!");
    }
    println!("I love {}s!!!", inside);
}

pub fn run() {
    println!("p059_option_unwrap >>>>>>>>");
    let food = Some("chicken");
    let snake = Some("snake1"); // 传 snake 会 panic
    let void: Option<&str> = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("bird");
    let _nothing: Option<&str> = None;

    give_princess(bird);
    //give_princess(nothing); // 这一句会 panic
    println!("\n");
}
