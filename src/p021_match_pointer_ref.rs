pub fn run(){
    println!("p021_match_pointer_ref >>>>>>>>");
    let reference = &4;
    match reference {
        &val => println!("Got a value via destructuring: {:?}", val)
    }

    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let _not_a_reference = 3;
    let ref _is_a_reference = 3;

    println!("_value: {:?}, ref: {:?}", _not_a_reference, _is_a_reference);

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10, mut_value = {:?}", m);
        }
    }

    println!("now mut_value = {:?}", mut_value);

    println!("\n");
}