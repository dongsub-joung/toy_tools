use std::{io::{self}};

mod user;
mod common;

use user::*;
use crate::common::{Files};


fn join(db: DB, node: u128) -> DB{
    // @todo check inputed String on sec.
    println!("ID");
    let mut id= String::new();
    io::stdin().read_line(&mut id).unwrap();
    
    println!("PW");
    let mut pw= String::new();
    io::stdin().read_line(&mut pw).unwrap();

    DB::add(User::new(id, pw, node), db)
}

fn main() {
    // About String menu
    const USER_MODE: &'static str   = "1.. Users Mode";
    const ADMIN_MODE: &'static str    = "2.. Admin Mode";
    const JOIN: &'static str          = "3.. Join";
    // const UNJOINED_USER: &'static str = "4.. Unjoined Users";

    // About User
    let me= User::dummy();
    let mut user_DB= DB::new(me);
    let mut user_node=0;

    // About Storage 
    let mut storage= 50.00;
    let mut used= 0.00;
    let remained= storage-used;

    // About File
    let mut file_DB: Vec<Files>= Vec::new();
    // let mut file_log: Vec<&Vec<Files>>= Vec::new();

    // About Admin
    let mut CODE= String::from("1234");
    
    // @todo hashed;

    loop{
        println!("{}", USER_MODE);
        println!("{}", ADMIN_MODE);
        println!("{}", JOIN);
        // @todo println!("{}", UNJOINED_USER);

        let mut buf= String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let number: u8= buf.trim().parse().unwrap();
        match number {
            1 => {
                println!("Usage: {}GB", remained);

                println!("{}", USER_MODE);
                let menus= "1. Download File
2. Upload File
3. Delete File
Select a Service you use";
                    
                    println!("{}", menus);
                    let select= common::inputing();
                    match select{
                        1 => {
                            let (title, path)= common::download(&file_DB);
                            println!("{}", title);
                            println!("{}", path);
                        },

                        2 => {
                            let file_data= common::upload();
                            used-= &file_data.volum;    
                            file_DB.push(file_data);
                        },

                        3 => {
                            file_DB= common::delete(file_DB);
                        },
                        _ => println!("Out range"),
                    }
            },
            
            2 => {
                let mut inputing_str= String::new();
                io::stdin().read_line(&mut inputing_str).unwrap();
                let inputing_str= inputing_str.trim().to_string();
                if CODE == inputing_str{
                    println!("Usage: {}GB", remained);
    
                    let menus= "1. Modification of Admin Code
2. Sharding";
    
                    println!("{}", ADMIN_MODE);
                    
                    println!("{}", menus);
                    let select= common::inputing();
                    match select {
                        1 => {
                            let mut codes= String::new();
                            io::stdin().read_line(&mut codes).unwrap();
                            let code= codes.trim().to_string();
                            CODE= code;
                        },

                        2 => {
                            storage += 50.00;
                        },

                        _ => println!("Out Range"),
                    }
                } else {
                    println!("Inncorrect");
                }
            },

            3 => {
                println!("{}", JOIN);
                user_DB= join(user_DB, user_node);
                user_node+=1;
            },

            _ => panic!("out of range")
        }
    }
}

