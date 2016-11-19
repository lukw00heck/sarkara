use seckey::Bytes;
use super::StreamCipher;


/// HC256.
///
/// # Example(process)
/// ```
/// # extern crate rand;
/// # extern crate sarkara;
/// # fn main() {
/// use rand::{ Rng, thread_rng };
/// use sarkara::stream::{ HC256, StreamCipher };
///
/// // ...
/// # let mut rng = thread_rng();
/// # let mut pass = vec![0; HC256::key_length()];
/// # let mut nonce = vec![0; HC256::nonce_length()];
/// # let mut data = vec![0; 1024];
/// # rng.fill_bytes(&mut pass);
/// # rng.fill_bytes(&mut nonce);
/// # rng.fill_bytes(&mut data);
///
/// let cipher = HC256::new(&pass);
/// let ciphertext = cipher.process(&nonce, &data);
/// let plaintext = cipher.process(&nonce, &ciphertext);
/// assert_eq!(plaintext, data);
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct HC256 {
    key: Bytes
}

impl StreamCipher for HC256 {
    fn new(key: &[u8]) -> Self where Self: Sized {
        HC256 { key: Bytes::new(key) }
    }

    #[inline] fn key_length() -> usize where Self: Sized { 32 }
    #[inline] fn nonce_length() -> usize where Self: Sized { 32 }

    fn process(&self, nonce: &[u8], data: &[u8]) -> Vec<u8> {
        let mut output = vec![0; data.len()];
        ::hc256::HC256::new(&self.key, nonce)
            .process(data, &mut output);
        output
    }
}
