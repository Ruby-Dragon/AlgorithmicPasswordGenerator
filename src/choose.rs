pub mod choose{
    use std::io::repeat;
    use std::ops::Add;
    use crate::input::input::PasswordInput;
    use crate::algorithms;
    use crate::algorithms::abbreviate::abrv;
    use crate::algorithms::swap::swap_in_word;
    use crate::algorithms::vowel_nt::strip_vowels;

    pub fn run_algorithms(user_input: PasswordInput) -> String {
        let mut final_password = String::new();

        let mut x : usize = 0;
        //look through each string in user_input
        while x < user_input.length as usize
        {
            //check which algorithm the user wants the string to use
            match user_input.algorithms[x]{
                1 => final_password.push_str(&*strip_vowels(user_input.words[x].clone())),
                2 => final_password.push_str(&*abrv(user_input.words[x].clone())),
                3 => final_password.push_str(&*swap_in_word(user_input.words[x].clone())),
                _=> final_password.push_str("1")    //if the user enters an invalid answer, then the 1 algorithm will be used
            };

            x+=1;
        }

        return final_password;
    }

}