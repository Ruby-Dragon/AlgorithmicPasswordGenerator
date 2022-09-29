pub mod choose{
    use std::io::repeat;
    use std::ops::Add;
    use crate::input::input::PasswordInput;

    pub fn run_algorithms(user_input: PasswordInput) -> String {
        let mut final_password = String::new();

        let mut x : usize = 0;
        while x < user_input.length as usize
        {
            //do stuff
            match user_input.algorithms[x]{
                1 => final_password.push_str("1"),
                2 => final_password.push_str("2"),
                3 => final_password.push_str("3"),
                _=> final_password.push_str("1")
            };

            x += 1;
        }

        return final_password;
    }

}