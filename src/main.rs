use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut text = String::new();
    let mut key = String::new();
    let mut operation = String::new();

    print!("Enter the text: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut text)?;

    print!("Enter the key: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut key)?;

    print!("Enter the operation (encrypt/decrypt): ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut operation)?;

    let key: i8 = key.trim().parse().expect("Please type a number!");

    match operation.trim() {
        "encrypt" => {
            let encrypted = caesar_cipher(&text.trim(), key);
            println!("Encrypted: {}", encrypted);
        },
        "decrypt" => {
            let decrypted = caesar_cipher(&text.trim(), -key);
            println!("Decrypted: {}", decrypted);
        },
        _ => println!("Invalid operation! Please enter 'encrypt' or 'decrypt'."),
    }

    Ok(())
}

fn caesar_cipher(input: &str, key: i8) -> String {
    let key = ((key % 26) + 26) % 26; // Ensure key is between 0 and 25
    input.chars().map(|char| {
        if char.is_ascii_alphabetic() {
            let base = if char.is_ascii_lowercase() { 'a' as u8 } else { 'A' as u8 };
            let offset = (char as u8 - base + key as u8) % 26;
            (base + offset) as char
        } else {
            char
        }
    }).collect()
}