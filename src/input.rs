pub mod input{

    //struct used to return data collected for word sets and algorithm choices
    pub struct PasswordInput{
        pub words:Vec<String>,
        pub algorithms:Vec<i32>,
        pub length: i32
    }

    //Get all the words and algorithm choices for each word set
    pub fn get_words() -> PasswordInput {

        //vectors used to hold the finished data
        let mut output_vector = Vec::new();
        let mut algorithm_vector = Vec::new();

        //keep asking user for words until user is done
        'word: loop {
            let mut word = String::new();

            //prompt user for words
            println!("Enter one set of words: ");
            std::io::stdin().read_line(&mut word).unwrap();
            //check if user is done
            if word.trim() == "-d"
            {
                println!("{word}");
                break 'word; //exit loop
            }
            //add to list of word sets
            output_vector.push(word);

            //ask user which algorithm to use for this word set
            println!("What algorithm should be used for this set(1-3)? ");
            let mut number_input= String::new();
            std::io::stdin().read_line(&mut number_input).unwrap();

            //turn string input into i32 and check for user error
            match number_input.trim() {
                "1" => algorithm_vector.push(1),
                "2" => algorithm_vector.push(2),
                "3" => algorithm_vector.push(3),
                _ => {println!("That is not a recognised input, defaulting to 1"); algorithm_vector.push(1)},
            }
        }

        //put data into struct to be returned
        let finished = PasswordInput {
            words : output_vector.clone(),
            algorithms : algorithm_vector,
            length : output_vector.len() as i32
        };

        //return collected data
        return finished;
    }
}