use std::io::{self, Write, Read};
use sha2::{Sha256, Sha512, Digest};

struct User<T, U>{
    id: T,
    pw: U,
}

fn main() {
    // root id, pw
    let hashed= hashing(String::from("123"));
    let root= User{
        id: "jds",
        pw: hashed,
    };

    let inputed_id= inputing();

    if root.id == inputed_id.as_str(){
        println!("OK");

        let inputed_pw= inputing();
        if root.pw == hashing(inputed_pw){
            println!("ALL DONE");
        } else {
            panic!("PW ERR");
        }
    } else {
        panic!("ID ERR");
    }
}

fn inputing()-> String{
    let mut buf= String::new();
    io::stdin().read_line(&mut buf);

    let result= buf.trim();
    result.to_string()
}

fn hashing(pw: String) -> String{
    let mut hasher= Sha256::new();
    hasher.update(pw.as_bytes());
    let result= hasher.finalize();
    let end= hex::encode(&result);

    end
}