use std::str::Bytes;
use sha2::{Sha256, Sha512, Digest};

#[cfg(test)]
fn hash_generator() -> {
    const HARD: &'static str= "String";
    let public_key= "abcd";
    let private_key= "jds"; 
    let plaintext= "Im_not_fuck_on_the_girl";

    let mut hasher= Sha256::new();
    let combine= format!("{}_{}", HARD,plaintext);

    hasher.update(combine.as_bytes());
    let mut result= hasher.finalize();
    let end= hex::encode(&result);

    end
}

mod testing{
    use super::*;

    #[test]
    fn test1(){

    }
}