pub fn run(){
    println!("p024_if_let >>>>>>>>");
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {}", i);
    };

    if let Some(i) = letter {
        println!("Letter: {}", i);
    }else{
        println!("Letter is None");
    };

    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("emoticon: {}", i);
    }else if i_like_letters {
        println!("Didn't match a number, let's go with a letter!");
    }else{
        println!("I don't like letters. let's go with an emoticon");
    };


    enum Foo{
        Bar,
        Baz,
        Qux(u32),
    }

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    };

    if let Foo::Bar = b {
        println!(" b is foobar");
    };

    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    };

    println!("\n");
}