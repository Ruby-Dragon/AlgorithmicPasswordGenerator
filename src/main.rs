mod input;
mod choose;

fn main() {
    println!("Algorithmic Password Generator. Enter -d when done.");
    let mut output_vector = input::input::get_words();

    for x in &output_vector.words{
        println!("{x}");
    }

    for x in &output_vector.algorithms{
        println!("{x}");
    }

    println!("{}", choose::choose::run_algorithms(output_vector))
}
