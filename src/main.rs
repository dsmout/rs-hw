use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    for i in args
    {
        println!("{}",i);
    }

    // println!("Hello, world!");
}
