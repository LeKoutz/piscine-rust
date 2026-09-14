#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let encrypted = encrypt(original);
    if encrypted == ciphered {
        Ok(())
    } else {
        Err(CipherError { expected: encrypted })
    }
}

fn encrypt(original: &str) -> String {
    let mut result = String::new();
    for c in original.chars() {
        if c.is_ascii_alphabetic() {
            if c.is_ascii_uppercase() {
                let offset = c as u8 - 'A' as u8;
                let mirror_psn = 25 - offset;
                let mirrored_c = ('A' as u8 + mirror_psn) as char;
                result.push(mirrored_c);
            } else {
                let offset = c as u8 - 'a' as u8;
                let mirror_psn = 25 - offset;
                let mirrored_c = ('a' as u8 + mirror_psn) as char;
                result.push(mirrored_c);
            }
        } else {
            result.push(c);
        }
    }
    result
}