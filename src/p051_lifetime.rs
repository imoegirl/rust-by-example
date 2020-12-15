fn print_ref<'a,  'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

fn faild_borrow<'a>(){
    let _x = 12;
}

fn print_one<'a>(x: &'a i32){
    println!("print_ont: x is {}", x);
}

fn add_one<'a>(x: &'a mut i32){
    *x += 1;
}

fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("print_multi: x: {}, y: {}", x, y);
}

fn pass_x<'a, 'b>(x: &'a i32, _:&'b i32) -> &'a i32 {x}

#[allow(dead_code)]
struct Owner(i32);

#[allow(dead_code)]
impl Owner {
    fn add_one<'a>(&'a mut self) { self.0 += 1; }
    fn print<'a>(&'a self) {
        println!("print: {}", self.0);
    }
}

#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}

#[derive(Debug)]
struct NamedBorrowed<'a>{
    x: &'a i32,
    y: &'a i32,
}

#[derive(Debug)]
enum Either<'a>{
    Num(i32),
    Ref(&'a i32),
}

impl <'a> Default for Borrowed<'a>{
    fn default() -> Self {
        Self {
            x: &10,
        }
    }
}

use std::fmt::Debug;
#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

#[allow(dead_code)]
fn print<T>(t: T) where T: Debug {
    println!("print: t is {:?}", t);
}

fn print_ref_debug<'a, T>(t: &'a T) where T: Debug + 'a {
    println!("print_ref: t is {:?}", t);
}

fn multiply<'a> (first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

static NUM: i32 = 18;

fn coerce_static<'a>(_:&'a i32) -> &'a i32 {
    &NUM
}

fn elided_input(x: &i32) {
    println!("elided_input: {}", x);
}

fn annotated_input<'a>(x: &'a i32) {
    println!("annotated_input: {}", x);
}

fn elided_pass(x: &i32) -> &i32 { x }
fn annotated_pass<'a>(x: &'a i32) -> &'a i32 { x }


pub fn run(){
    println!("p051_lifetime >>>>>>>>");
    let (four, nine) = (4, 9);
    print_ref(&four, &nine);
    faild_borrow();
    //--------------------------
    let x = 7;
    let y = 9;
    print_one(&x);
    print_multi(&x, &y);
    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
    // --------------------------

    let x = 18;
    let y = 15;
    let single = Borrowed{x: &x};
    let double = NamedBorrowed{x: &x, y: &y};
    let reference = Either::Ref(&x);
    let number = Either::Num(y);
    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
    // ---------------------------

    let b:Borrowed = Default::default();
    println!("b is {:?}", b);

    // ----------------------------
    let x = 7;
    let ref_x = Ref(&x);
    print_ref_debug(&ref_x);
    print(ref_x);

    // ----------------------------
    let first = 2;
    {
        let second = 3;
        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    }

    // ----------------------------
    {
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);
    }

    {
        let lifetime_num = 9;
        let coerced_static = coerce_static(&lifetime_num);
        println!("coerced_static: {}", coerced_static);
    }

    println!("Num:{} stays accessible!", NUM);
    let x = 3;
    elided_input(&x);
    annotated_input(&x);

    println!("elided_pass: {}", elided_pass(&x));
    println!("annotated_pass: {}", annotated_pass(&x));

    println!("\n");
}