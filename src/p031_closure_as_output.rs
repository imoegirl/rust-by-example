fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a : {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();
    move || println!("This is a :{}", text)
}

pub fn run() {
    println!("p031_closure_as_output >>>>>>>>");
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();

    fn_plain();
    fn_mut();
    println!("\n");
}
