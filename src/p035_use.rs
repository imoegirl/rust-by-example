use deeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

pub fn run() {
    println!("p035_use >>>>>>>>");
    other_function();

    println!("Entering block");
    {
        other_function();
        use deeply::nested::function;
        function();
        println!("Leaving block");
    }

    function();
    println!("\n");
}
