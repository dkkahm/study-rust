use aes::Aes256;
use block_modes::{BlockMode, Cbc, block_padding::Pkcs7};
use rand::Rng;
use sha3::{Digest, Sha3_256};

use crate::configuration::get_configuration;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;
const IV_SIZE: usize = 16;

fn generate_token_salt() -> [u8; 16] {
    let mut key = [0u8; 16];
    rand::thread_rng().fill(&mut key);
    key
}

// pub fn encode_token(claim: &str) -> Result<String, anyhow::Error> {
//     let mut token = claim.to_string();

//     let token_secret = get_configuration()
//         .map_err(|e| anyhow::anyhow!("{}", e))?
//         .application.token_secret;

//     let token_salt = generate_token_salt();

//     let mut key_buffer: Vec<u8> = token_salt.to_vec();
//     let mut stream = Aes256Ctr::new(cipher.clone(), &[0u8; 16]);
//     stream.apply_keystream(&mut key_buffer);

//     token.push(':');
//     token.push(format!("{:x}", key_buffer));

//     let mut hasher = Sha3_256::new();
//     hasher.update(token.as_bytes());
//     let hash = hasher.finalize();
//     let hash = format!("{:x}", hash);

//     token.push(':');
//     token.push(hash);

//     Ok(token)
// }

// pub fn get_claim_from_token(token: &str) -> Result<String, anyhow::Error> {
//     let token_parts = token.rsplitn(2, ':');
//     let claim_and_salt_in_token = token_parts.next().or_else(|| anyhow::anyhow!("Invalid token format"))?;
//     let hash_in_token = token_parts.next().or_else(|| anyhow::anyhow!("Invalid token format"))?;

//     let mut hasher = Sha3_256::new();
//     hasher.update(claim_and_salt_in_token.as_bytes());
//     let hash = hasher.finalize();
//     let expected_hash = format!("{:x}", hash);

//     if hash_in_token != expected_hash {
//         return anyhow::anyhow!("Invalid token hash");
//     }

//     let claim_and_salt_parts_in_token = claim_and_salt_in_token.split(':');
//     let claim_in_token = claim_and_salt_parts_in_token.next().or_else(|| anyhow::anyhow!("Invalid token format"))?;
//     let salt_in_token = claim_and_salt_parts_in_token.next().or_else(|| anyhow::anyhow!("Invalid token format"))?;

//     let token_secret = get_configuration()
//         .map_err(|e| anyhow::anyhow!("{}", e))?
//         .application.token_secret;

//     let cipher = match Aes256::new_from_slice(token_secret.as_bytes()) {
//         Ok(cipher) => cipher,
//         Err(e) => anyhow::Error(e),
//     };

//     let mut buffer = salt_in_token.as_bytes().to_vec();
//     let mut stream = Aes256Ctr::new(cipher, &[0u8; 16]);

//     stream.apply_keystream(&mut buffer);

//     if let Err(e) = String::from_utf8(buffer) {
//         return anyhow::Error(e);
//     }

//     Ok(claim.to_string())
// }

fn encrypt_message(message: &str, password: &str) -> Result<String, anyhow::Error> {
    let password = password.as_bytes();

    // Generate a random IV
    let mut iv = [0u8; IV_SIZE]; // IV size for AES-256
    rand::thread_rng().fill(&mut iv);

    // Create the cipher instance
    let cipher = Cbc::<Aes256, Pkcs7>::new_from_slices(password, &iv)?;

    // Encrypt the message
    let ciphertext = cipher.encrypt_vec(message.as_bytes());

    // Concatenate IV and ciphertext
    let mut result = iv.to_vec();
    result.extend(ciphertext);

    Ok(hex::encode(result))
}

fn decrypt_message(ciphertext: &str, password: &str) -> Result<String, anyhow::Error> {
    if ciphertext.len() < IV_SIZE {
        return Err(anyhow::anyhow!("Invalid ciphertext length"));
    }

    let password = password.as_bytes();

    let ciphertext_bytes = hex::decode(ciphertext.as_bytes())
        .or_else(|e| Err(anyhow::anyhow!("{}", e)))?;

    // Split the ciphertext into IV and ciphertext
    let (iv, ciphertext) = ciphertext_bytes.split_at(IV_SIZE);

    // Create the cipher instance
    let cipher = Cbc::<Aes256, Pkcs7>::new_from_slices(password, iv)?;

    // Decrypt the ciphertext
    let plaintext = cipher.decrypt_vec(&ciphertext)?;

    // Convert the plaintext to a string and remove padding
    let plaintext_str = String::from_utf8_lossy(&plaintext).into_owned();
    let plaintext_str = plaintext_str.trim_end_matches(char::from(0));

    Ok(plaintext_str.to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encypt_and_decrypt() {
        let claim = "test";
        let password = "0123456789abcdef0123456789abcdef";
        let encypted_message = encrypt_message(claim, password).unwrap();
        let decrypted_message = decrypt_message(&encypted_message, password).unwrap();
        
        // Assert
        assert_eq!(claim.to_string(), decrypted_message);
    }
}