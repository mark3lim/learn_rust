/* Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay is added, 
 * so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). 
 * Keep in mind the details about UTF-8 encoding!
 */
pub mod q2_string {
    use std::io;
    use std::io::Write;
    use std::str::Chars;

    pub fn test() {
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        let mut pig_latin = String::new();

        print!("Input a word: ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin()
            .read_line(&mut pig_latin)
            .expect("Failed to read line");
        
        let pig_latin = pig_latin.trim();
        
        let mut pig_word = String::new();
        let first_char: char = pig_latin.chars().next().unwrap();
        if vowels.contains(&first_char) {
            pig_word.push_str(pig_latin);
            pig_word.push_str("-hay");
            
        } else {
            let mut chars: Chars = pig_latin.chars();
            chars.next().unwrap(); // remove the first character
            pig_word.push_str(chars.as_str());
            pig_word.push('-');
            pig_word.push(first_char);
            pig_word.push_str("ay");
        }
        
        println!("Pig Latin: {}", pig_word);
    }
}