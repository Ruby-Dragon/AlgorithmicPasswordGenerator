mod input;
mod choose;
mod algorithms;

fn main() {
    //print prompts and get input
    println!("Algorithmic Password Generator. Enter -d when done.");
    let mut input_words = input::input::get_words();

    //output finished code
    println!("Generated Password:");
    println!("{}", choose::choose::run_algorithms(input_words));
}
