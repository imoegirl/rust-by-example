fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }

    pub fn indirect_call() {
        println!("called `my::indirect_call()`, that\n>");
        self::function();
        function();
        self::cool::function();
        super::function();
        {
            use crate::p036_super_self::cool::function as root_function;
            root_function();
        }
    }
}

pub fn run() {
    println!("p06_super_self >>>>>>>>");
    my::indirect_call();
    println!("\n");
}
