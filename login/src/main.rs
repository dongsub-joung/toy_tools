use std::io::{self, Write, Read};
use sha2::{Sha256, Sha512, Digest};

struct User<T, U>{
    id: T,
    pw: U,
}

fn main() {
    // root id, pw
    let hashed_str= hashing(String::from("123"));
    let root= User{
        id: "jds",
        pw: hashed_str,
    };

    let inputed_id= inputing();

    if root.id == inputed_id.as_str(){
        let inputed_pw= inputing();
        if root.pw == hashing(inputed_pw){
            println!("Done");
        }
    }
}

fn inputing()-> String{
    let mut buf= String::new();
    io::stdin().read_line(&mut buf);

    buf
}

fn hashing(pw: String) -> String{
    let mut hasher= Sha256::new();
    hasher.update(pw.as_bytes());
    let result= hasher.finalize();
    let end= hex::encode(&result);

    end
}