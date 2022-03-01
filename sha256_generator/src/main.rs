use sha2::{Sha256, Sha512, Digest};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::os::raw;

fn main(){
    let mut raw_data= String::new();
    raw_data= getRawFileData();
    
    // let hex_256= generateSha256(&raw_data);
    // let hex_512= generateSha512(&raw_data);

    println!("{}", raw_data);
}

fn getRawFileData() -> String {
    let filename= "1.txt";
    let mut f= File::open(filename).expect("File not found");
    let mut contents= String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    contents
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