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

pub mod vowel_nt
{
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    //remove vowels from string
    pub fn strip_vowels(input : String) -> String{
        let mut final_string = String::new();

        //go through every letter in input
        for letter in input.trim().chars()
        {
            //interesting loop thing
            'skip: loop
            {
                //check if vowel
                for vowel in VOWELS
                {
                    if letter == vowel || letter == ' '
                    {
                        //skip code that adds the letter
                        break 'skip;
                    }
                }
                //add letter to final string
                final_string.push(letter);
                break 'skip;
            }

        }
        //return vowel-less string
        return final_string;
    }
}

pub mod abbreviate
{
    const UPPER : [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

    //abbreviate string - all capital letters in string
    pub fn abrv(input : String) -> String
    {
        let mut final_string = String::new();
        //look through each letter of input
        for letter in input.trim().chars()
        {
            'skip: loop
            {
                //check if letter is uppercase...
                for uppercase in UPPER
                {
                    if letter == uppercase
                    {
                        //...and add to final string
                        final_string.push(letter);
                        break 'skip;
                    }
                }
                break 'skip;
            }

        }
        return final_string;
    }
}

pub mod swap
{
    pub fn swap_in_word(input : String) -> String
    {
        let mut temp_string = String::new();
        //only uses the first word of whatever is passed in
        for letter in input.trim().chars()
        {
            //remove all other words in input
            if letter == ' '
            {
                break;
            }
            temp_string.push(letter);
        }

        //get first and last letters
        let first = temp_string.chars().nth(0).expect("Error: Input words contain non-ascii characters. Please do not use non-ascii characters.");
        let last = temp_string.chars().nth(temp_string.len() -1).expect("Error: Input words contain non-ascii characters. Please do not use non-ascii characters.");

        //add last letter as first letter in new string
        let mut final_string = String::new();
        final_string.push(last);

        //fill string with the middle letters
        let mut count = 0;
        for letter in temp_string.chars()
        {
            if count != 0 && count != temp_string.len() -1
            {
                final_string.push(letter);
            }
            count+=1;
        }
        //add first string as last
        final_string.push(first);

        return final_string;
    }
}