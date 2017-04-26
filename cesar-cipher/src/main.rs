use std::io;

const A_INDEX: i8 = 'a' as i8;
const Z_INDEX: i8 = 'z' as i8;
const ALPHABET_LEN: i8 = 26;

fn main() {
    println!("Enter text:");
    let input_text = get_input();

    println!("Enter key: ");
    let key:i8 = match get_input().parse() {
    	Ok(val) => { val },
    	Err(error) => panic!("Can not parse input as number, {}", error)
    };

    let cipher_string = cipher(&input_text, key);
    println!("Cipher {}", cipher_string);

    println!("decipher {}", cipher(&cipher_string, -key));
}

fn cipher(input: &String, key: i8) -> String {
	input.chars()
    	.map(|c| c.to_lowercase().to_string())
    	.map(|letter| cipher_letter(letter.as_bytes()[0], key))
    	.collect::<String>()
}

fn cipher_letter(letter_bytes: u8, key: i8) -> char {
    if !is_letter(letter_bytes) {
        letter_bytes as char
    } else {
        cipher_alphabetic(letter_bytes as i8 - A_INDEX, key)
    }
}

fn cipher_alphabetic(char_index: i8, key: i8) -> char {
    let decoded_char = (char_index + key) % ALPHABET_LEN;
    let decoded_char = if decoded_char > ALPHABET_LEN {
        decoded_char - ALPHABET_LEN
    } else if decoded_char < 0 {
        decoded_char + ALPHABET_LEN
    } else {
        decoded_char
    };

    (decoded_char + A_INDEX) as u8 as char
}

fn is_letter(letter_bytes: u8) -> bool {
    letter_bytes as i8 >= A_INDEX && letter_bytes as i8 <= Z_INDEX
}

fn get_input() -> String {
	let mut buf = String::new();
    io::stdin()
    	.read_line(&mut buf)
    	.expect("Failed to read line");
    buf.trim().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_ignore_non_alphabetic_characters() {
        assert_eq!(cipher_letter(' ' as u8, 2), ' ');
        assert_eq!(cipher_letter('1' as u8, 2), '1');
        assert_eq!(cipher_letter('"' as u8, 2), '"');
    }

    #[test]
    fn should_cipher_letter() {
        assert_eq!(cipher_letter('a' as u8, 2), 'c');

        assert_eq!(cipher_letter('z' as u8, 1), 'a');
        assert_eq!(cipher_letter('z' as u8, 2), 'b');
    }

    #[test]
    fn should_decipher_letter() {
        assert_eq!(cipher_letter('z' as u8, -2), 'x');
        assert_eq!(cipher_letter('c' as u8, -2), 'a');

        assert_eq!(cipher_letter('a' as u8, -1), 'z');
        assert_eq!(cipher_letter('b' as u8, -2), 'z');
        assert_eq!(cipher_letter('a' as u8, -3), 'x');
    }

    #[test]
    fn integration_test() {
        assert_eq!(cipher(&"ala ma kota! 12345._AS".to_string(), 2), "cnc oc mqvc! 12345._cu");
        assert_eq!(cipher(&"cnc oc mqvc! 12345._cu".to_string(), -2), "ala ma kota! 12345._as");
    }
}
