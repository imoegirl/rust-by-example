use std::convert::TryFrom;
use std::convert::TryInto;

// try from 和 try into，是在类型转换中，可能会出错的情况下使用，
// 如果没问题，那就返回目标类型，如果有问题，那就返回错误

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

pub fn run() {
    println!("p012_tryfrom_tryinto >>>>>>>>");
    let result = EvenNumber::try_from(8);
    println!("try_from 8: {:?}", result);
    let result = EvenNumber::try_from(5);
    println!("try_from 5: {:?}", result);

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    println!("8i32 try_into: {:?}", result);

    let result: Result<EvenNumber, ()> = 5i32.try_into();
    println!("5i32 try_into: {:?}", result);
    println!("\n");
}
