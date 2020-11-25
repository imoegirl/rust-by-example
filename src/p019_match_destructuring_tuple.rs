
pub fn run(){
    println!("p019_match_destructuring_tuple >>>>>>>>");
    let triple = (0, -2, 3);
    
    match triple {
        (0, y, z) => println!("First is '0', y is {:?}, z is {:?}", y, z),
        (1, ..) => println!("First is 1 and teh rest doesn't matter"),
        _ => println!("It doesn't matter what they are"),
    }

    println!("\n");
}