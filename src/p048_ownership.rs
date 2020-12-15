
fn create_box(){
    let _box1 = Box::new(3i32);
}

fn destroy_box(c: Box<i32>){
    println!("Destroying a box that contains {}", c);
}

pub fn run(){

    println!("p048_ownership >>>>>>>>");

    let _box2 = Box::new(5i32);
    {
        let _box3 = Box::new(4i32);
    }

    for _ in 0u32..1_1000 {
        create_box();
    }

    // ------------------
    let x = 5u32;
    let y = x;
    println!("x is {}, and y is {}", x, y);
    let a = Box::new(5i32);
    let b = a; // move a to b
    destroy_box(b);

    // -------------------
    let immutable_box = Box::new(5u32);
    println!("immutable_box contains {}", immutable_box);
    let mut mutable_box = immutable_box; // move immutable_box to mutable_box, make it mutable
    *mutable_box = 4;
    println!("mutable_box now contains {}", mutable_box);

    println!("\n");

}