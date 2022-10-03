/*
Copyright (C) 2022 Ruby-Dragon

This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

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