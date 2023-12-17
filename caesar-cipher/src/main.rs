// ---------------------
// V1 CAESAR CIPHER
// ---------------------

// fn encrypt(_text: &str, _shift: i32) -> String {
//     let alphabet: Vec<char> = ('a'..='z').collect();
//     let mut result: String = String::new();

//     for c in _text.chars() {
//         if let Some(index) = alphabet.iter().position(|&x| x == c) {
//             let shift: usize = (index as i32 + _shift).rem_euclid(alphabet.len() as i32) as usize;
//             result.push(alphabet[shift]);
//         } else {
//             result.push(c);
//         }
//     }
//     result
// }

// fn decrypt(_text: &str, _shift: i32) -> String {
//     let alphabet: Vec<char> = ('a'..='z').collect();
//     let mut result: String = String::new();

//     for c in _text.chars() {
//         if let Some(index) = alphabet.iter().position(|&x| x == c) {
//             let shift: usize = (index as i32  - _shift).rem_euclid(alphabet.len() as i32) as usize;
//             result.push(alphabet[shift]);
//         } else {
//             result.push(c);
//         }
//     }
//     result
// }

// ---------------------
// V2 CAESAR CIPHER
// ---------------------

fn caesar_cipher(text: &str, shift: i32) -> String {
    text.chars()
        .map(|c| match c.is_alphabetic() {
            true => {
                let base = if c.is_lowercase() { 'a' } else { 'A' } as i32;
                let offset = c as i32 - base;
                let shifted = (offset + shift).rem_euclid(26) as u8;
                (base + shifted as i32) as u8 as char
            }
            false => c,
        })
        .collect()
}

fn main() {
    let input: &str = "this is a caesar cipher";
    let shift: i32 = 3;
    let encrypted_text = caesar_cipher(input, shift);
    let decrypted_text = caesar_cipher(&encrypted_text, -shift);
    println!(
        "Encrypted Text: '{}', \nDecrypted Text: '{}'",
        encrypted_text, decrypted_text
    );
}
