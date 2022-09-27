mod input;

fn main() {
    println!("Hello, world!");
    let mut output_vector = input::input::get_words();

    for x in &output_vector.words{
        println!("{x}");
    }

    for x in &output_vector.algorithms{
        println!("{x}");
    }
}
