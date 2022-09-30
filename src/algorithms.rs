pub mod vowel_nt
{
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    pub fn strip_vowels(input : String) -> String{
        let mut final_string = String::new();

        for letter in input.chars()
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
        for letter in input.chars()
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
        let mut final_string = String::new();

        return final_string;
    }
}