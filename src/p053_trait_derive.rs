#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        Centimeters(self.0 as f64 * 2.54)
    }
}

struct Seconds(i32);

pub fn run(){
    println!("p053_trait_derive >>>>>>>>");
    let _one_second = Seconds(1);
    let foot = Inches(12);
    println!("One foot equals {:?}", foot);
    let meter = Centimeters(100.0);
    let cmp = if foot.to_centimeters() < meter {
        "smaller"
    }else{
        "bigger"
    };

    println!("One foot is {} than one meter.", cmp);

    println!("\n");
}