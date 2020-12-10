mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called `split_file::function()`");
}

fn private_function() {
    println!("called `split_file::private_function()`")
}

pub fn indirect_access() {
    println!("called `split_file::indirect_access()`, that\n>");
    private_function();
}
