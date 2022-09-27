pub mod input{
    pub fn get_words() -> Vec<String> {
        let mut output_vector = Vec::new();
        'word: loop {
            let mut word = String::new();
            std::io::stdin().read_line(&mut word).unwrap();
            if word.trim() == "-d"
            {
                println!("{word}");
                break 'word;
            }
            output_vector.push(word);
        }

        return output_vector;
    }
}