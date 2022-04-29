use std::str::Bytes;
use sha2::{Sha256, Sha512, Digest};

#[cfg(test)]
fn hash_generator() -> String{
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
        // The aother
        let mut hasher= Sha256::new();
        let plain= "String_Im_not_fuck_on_the_girl";
        hasher.update(plain.as_bytes());
        let mut result= hasher.finalize();
        let end= hex::encode(&result);
        
        assert_eq!(end, hash_generator());
    }
}