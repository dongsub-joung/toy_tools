use sha2::{Sha256, Sha512, Digest};
use std::env;
use std::fs::File;
use std::io::Write;

fn main(){
    let mut raw_data= String::new();
    raw_data= "hellow world".to_string();
    
    let hex_256= generateSha256(&raw_data);
    let hex_512= generateSha512(&raw_data);

    
}

fn generateSha256(data: &String) -> String{
    let mut hasher= Sha256::new();
    
    hasher.update(data.as_bytes());
    
    let result= hasher.finalize();
    let end= hex::encode(&result);
    
    format!("sha256: {}", end)
}

fn generateSha512(data: &String) -> String{
    let mut hasher= Sha512::new();
    
    hasher.update(data.as_bytes());

    let result= hasher.finalize();
    let end= hex::encode(&result);

    format!("sha512: {}", end)
}