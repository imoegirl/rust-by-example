struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

pub fn run(){
    println!("p055_trait_drop >>>>>>>>");
    let _a = Droppable{name: "a"};
    {
        let _b = Droppable{name: "b"};
        {
            let _c = Droppable{ name: "c"};
            let _d = Droppable{name: "d"};
            println!("Exiting block B");
        }

        println!("just exited block B");
        println!("Exiting block A");
    }
    println!("Just exited block A");
    println!("End of the main function");

    println!("\n");
}