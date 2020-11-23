fn for_include_start_exclusive_end() {
    for n in 1..3 {
        println!("1 .. 3, now is : {}", n);
    }
}

fn for_include_start_and_end() {
    for n in 1..=3 {
        println!("1 ..= 3, now is :{}", n);
    }
}

// iter 会对集合中的元素进行借用，并不会获取所有权，在迭代结束后，集合中的元素未变，还可以继续使用
fn for_in_iter() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Frank" => println!("iter, There is a restacean amount us: {}", name),
            _ => println!("iter, oh, friend: {}", name),
        }
    }
    println!("iter, names: {:?}", names);
}

fn for_in_into_iter() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Frank" => println!("into_iter, There is a restacean amount us: {}", name),
            _ => println!("into_iter, oh, friend: {}", name),
        }
    }
    // println!("{:?}", names); // ERROR, names 已经被消耗
}

// 可变借用每一个元素，然后可以修改每一个元素的值
fn for_in_iter_mut() {
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a restacean amoung us!",
            _ => "Hello",
        }
    }
    println!("iter_mut, names: {:?}", names);
}

fn for_in_iter_mut2() {
    let mut scores = vec![1, 2, 3, 4, 5];
    println!("Source Score: {:?}", scores);
    for score in scores.iter_mut() {
        *score = *score * 2;
    }
    println!("Scores: {:?}", scores);
}

pub fn run() {
    println!("p017_for_and_range >>>>>>>>");
    for_include_start_exclusive_end();
    for_include_start_and_end();
    for_in_iter();
    for_in_into_iter();
    for_in_iter_mut();
    for_in_iter_mut2();
    println!("\n");
}
