use std::io;

fn main() {
    println!("Enter text:");
    let input_text = get_input();

    println!("Enter key: ");
    let key:u8 = match get_input().parse() {
    	Ok(val) => { val },
    	Err(error) => panic!("Can not parse input as number, {}", error)
    };

    let cipher_string = cipher(&input_text, key);
    println!("Cipher {}", cipher_string);
}


fn cipher(input: &String, key: u8) -> String {
	input.chars()
    	.map(|c| c.to_lowercase().to_string())
    	.map(|letter| cipher_letter(letter.as_bytes()[0], key))
    	.collect::<String>()
}

fn cipher_letter(letter_bytes: u8, key: u8) -> char {
	let a_index = 'a' as u8;
	let alphabet_len = 26;
	let is_letter = letter_bytes >= a_index && letter_bytes <= 'z' as u8;
	if !is_letter {
		letter_bytes  as char
	} else {
		let char_index = letter_bytes - a_index;
		((char_index + key) % alphabet_len + a_index) as char
	}
}

fn get_input() -> String {
	let mut buf = String::new();
    io::stdin()
    	.read_line(&mut buf)
    	.expect("Failed to read line");
    buf.trim().to_string()
}
