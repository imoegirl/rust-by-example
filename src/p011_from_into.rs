use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// from 和 into 就是像是孪生，实现了From trait，也就拥有了 into，用法如下
// 个人理解, from 是将某个东西，转为自己
// 而 into，是将自己转为某个东西，正在是反过来
// 但是最终结果是一样的
pub fn run() {
    println!("p011_from_into >>>>>>>>");

    let num = Number::from(30);
    println!("My number is {:?}", num);

    let i_num = 5;
    let num: Number = i_num.into();
    println!("My number is: {:?}", num);

    println!("\n");
}
