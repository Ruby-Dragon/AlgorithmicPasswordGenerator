pub mod vowel_nt
{
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    pub fn strip_vowels(input : String) -> String{
        let mut final_string = String::new();

        for letter in input.trim().chars()
        {
            'skip: loop
            {
                for vowel in VOWELS
                {
                    if letter == vowel || letter == ' '
                    {
                        break 'skip;
                    }
                }
                final_string.push(letter);
                break 'skip;
            }

        }

        return final_string;
    }
}

pub mod abbreviate
{
    const UPPER : [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

    pub fn abrv(input : String) -> String
    {
        let mut final_string = String::new();
        for letter in input.trim().chars()
        {
            'skip: loop
            {
                for uppercase in UPPER
                {
                    if letter == uppercase
                    {
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
            if letter == ' '
            {
                break;
            }
            temp_string.push(letter);
        }

        let first = temp_string.chars().nth(0).expect("Error: Input words contain non-ascii characters. Please do not use non-ascii characters.");
        let last = temp_string.chars().nth(temp_string.len() -1).expect("Error: Input words contain non-ascii characters. Please do not use non-ascii characters.");

        let mut final_string = String::new();
        final_string.push(last);

        let mut count = 0;
        for letter in temp_string.chars()
        {
            if count != 0 && count != temp_string.len() -1
            {
                final_string.push(letter);
            }
            count+=1;
        }
        final_string.push(first);

        return final_string;
    }
}