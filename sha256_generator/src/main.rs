use sha2::{Sha256, Sha512, Digest};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::os::raw;

// https://rust-lang-nursery.github.io/rust-cookbook/file/read-write.html
// https://stackoverflow.com/questions/31192956/whats-the-de-facto-way-of-reading-and-writing-files-in-rust-1-x

fn main(){
    let mut raw_data= String::new();
    let names= getFileNames();
    for i in 0..names.len(){
        raw_data= getRawFileData();

        let file_name= &names[i]; 
        let hex_256= generateSha256(&raw_data);
        let hex_512= generateSha512(&raw_data);
    
        let writeable= format!("{} \n{}\n{}", file_name, hex_256, hex_512);
        
        // save a each "{i}.txt"
        // 
    }


    println!("{}", raw_data);
}

fn getFileNames() -> Vec<String> {
    let mut v: Vec<String>= Vec::new();
    // get the file name in directory, then set v

    v
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