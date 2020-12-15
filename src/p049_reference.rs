fn eat_box_i32(boxed_i32: Box<i32>){
    println!("Destroying Box that contains {}", boxed_i32);
}

fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book){
    println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

fn new_edition(book: &mut Book){
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

struct Point {x: i32, y: i32, z: i32}

pub fn run(){
    println!("p049_reference >>>>>>>>");
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        let _ref_to_i32 = &boxed_i32;
        // eat_box_i32(boxed_i32) // when the value in borrow, you cant destroy it.
        borrow_i32(_ref_to_i32);
    }

    eat_box_i32(boxed_i32); // no borrowed, we can destroy the boxed_i32

    // ----------------------

    let immutablebook = Book {
        author: "Douglas Hofstadter",
        title: "Godel, Escher, Bach",
        year: 1979,
    };

    let mut mutabook = immutablebook;   // copy immutable to mutable
    borrow_book(&immutablebook);
    borrow_book(&mutabook);
    new_edition(&mut mutabook);
    // new_edition(&mut immutablebook); // can't borrow immutable value
    // -----------------------------
    let mut _mutable_integer = 7i32;
    {
        let large_integer = &_mutable_integer;
        // _mutable_integer = 50; // you can't change the value when there is a immutable borrow
        println!("Immutably borrowed {}", large_integer);
    }

    _mutable_integer = 3;

    // -------------------------------
    let mut point = Point { x: 0, y: 0, z: 0};
    {
        let borrowed_point = &point;
        let another_borrow = &point;

        println!("Point has coordinates: ({}, {}, {})", borrowed_point.x, another_borrow.y, point.z);
        println!("Point has corrdinates: ({}, {}, {})", borrowed_point.x, another_borrow.y, point.z);
    }

    {
        let mutable_borrow = &mut point;
        mutable_borrow.x = 5;
        mutable_borrow.y = 2;
        mutable_borrow.z = 1;

        // let y = &point.y;    // 存在可变借用，所以这里不能进行不可变借用
        println!("Point has corrdinates: ({}, {}, {})", mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);
    }

    let borrowed_point = &point;
    println!("Point now has corrdinates: ({}, {}, {})", borrowed_point.x, borrowed_point.y, borrowed_point.z);
    println!("\n");
}