/*

Each character on a computer is assigned a unique code and the preferred standard is ASCII
(American Standard Code for Information Interchange). For example, uppercase A = 65,
 asterisk (*) = 42, and lowercase k = 107.

A modern encryption method is to take a text file, convert the bytes to ASCII, then XOR each
byte with a given value, taken from a secret key. The advantage with the XOR function
is that using the same encryption key on the cipher text, restores the plain text;
for example, 65 XOR 42 = 107, then 107 XOR 42 = 65.

For unbreakable encryption, the key is the same length as the plain text message, and the key
is made up of random bytes. The user would keep the encrypted message and the encryption
key in different locations, and without both "halves", it is impossible to decrypt the message.

Unfortunately, this method is impractical for most users, so the modified method is to use
a password as a key. If the password is shorter than the message, which is likely, the key is repeated
cyclically throughout the message. The balance for this method is using a sufficiently long password key
for security, but short enough to be memorable.

Your task has been made easy, as the encryption key consists of three lower case characters.
Using p059_cipher.txt (right click and 'Save Link/Target As...'), a file containing the
encrypted ASCII codes, and the knowledge that the plain text must contain common English words,
decrypt the message and find the sum of the ASCII values in the original text.

*/

use math::crypto::*;
use std::fs::*;

pub fn pe59() {
    let text = read_to_string("p059_cipher.txt").unwrap();
    let ascii: Vec<u32> = text
        .split(',')
        .map(|s| s.parse::<u32>())
        .flatten()
        .collect();

    for a in 97..=122 {
        for b in 97..=122 {
            for c in 97..=122 {
                let password = vec![a, b, c];
                let decrypted = &xor_decimal_text(&ascii, &password);
                let cleartext = ascii_to_string(&decrypted);
                if cleartext.contains("Euler's") {
                    println!("{cleartext}");
                    println!("{}", decrypted.iter().sum::<u32>());
                    return;
                }
            }
        }
    }
}

fn ascii_to_string(vec: &Vec<u32>) -> String {
    let mut s = String::new();
    for n in vec {
        s.push(char::from_u32(*n).unwrap());
    }
    s
}
