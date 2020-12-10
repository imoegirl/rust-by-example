mod split_files;

fn function() {
    println!("called `function()`");
}

pub fn run() {
    println!("p037_split_file >>>>>>>>");
    split_files::function();
    function();
    split_files::indirect_access();
    split_files::nested::function();
    println!("\n");
}
