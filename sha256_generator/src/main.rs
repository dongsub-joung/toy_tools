use sha2::{Sha256, Sha512, Digest};
use hex_literal::hex;

fn main() {
    let mut hasher= Sha256::new();
    hasher.update(b"hello world");
    let result= hasher.finalize();

    // assert_eq!(result[..], hex!("
    // b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
    // ")[..]);
    println!("{:#?}", result[..]);
}