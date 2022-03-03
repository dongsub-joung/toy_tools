use sha2::{Sha256, Sha512, Digest};
use std::{env, string};
use std::fs::File;
use std::io::prelude::*;
use std::os::raw;

fn main(){
    let mut raw_data= String::new();
    let names= getFileNames();
    for i in 0..names.len(){
        let file_name= &names[i]; 
        raw_data= getRawFileData(file_name.clone());
        
        let hex_256= generateSha256(&raw_data);
        let hex_512= generateSha512(&raw_data);
    
        let writeable= format!("{} \n{}\n{}", file_name, hex_256, hex_512);

        let mut f= File::create("save.txt").expect("not found file in save");
        f.write_all(writeable.as_bytes()).expect("err to write");
    }
}

fn getFileNames() -> Vec<String> {
    let mut v: Vec<String>= Vec::new();
    
    // get the file name in directory, then set v
    v.push("1.txt".to_string());

    v
}

fn getRawFileData(filename: String) -> String {
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