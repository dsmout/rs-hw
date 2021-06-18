use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    let phrase = "Yellow Lorry";
    let sub_phrase = &phrase[..3];

    println!("{}", sub_phrase);
    for i in 0..9
    {
        println!("Hello {}",i);
    }

    // println!("Hello, world!");
}
