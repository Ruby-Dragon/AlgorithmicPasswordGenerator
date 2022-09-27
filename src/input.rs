pub mod input{

    pub struct PasswordInput{
        pub words:Vec<String>,
        pub algorithms:Vec<i32>
    }

    pub fn get_words() -> PasswordInput {
        let mut output_vector = Vec::new();
        let mut algorithm_vector = Vec::new();
        'word: loop {
            let mut word = String::new();
            std::io::stdin().read_line(&mut word).unwrap();
            if word.trim() == "-d"
            {
                println!("{word}");
                break 'word;
            }
            output_vector.push(word);

            println!("What algorithm should be used (1-3)? ");
            let mut number_input= String::new();
            std::io::stdin().read_line(&mut number_input).unwrap();
            match number_input.trim() {
                "1" => algorithm_vector.push(1),
                "2" => algorithm_vector.push(2),
                "3" => algorithm_vector.push(3),
                _ => {println!("That is not a recognised input, defaulting to 1"); algorithm_vector.push(1)},
            }
        }
        let finished = PasswordInput {
            words : output_vector,
            algorithms : algorithm_vector
        };

        return finished;
    }
}