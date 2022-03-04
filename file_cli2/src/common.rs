use std::{io::{self, stdin}, fs::File};

pub struct Files{
    pub name: String,
    pub volum: f64,
    pub path: String,
}

pub fn download(files: &Vec<Files>) -> (String, String){
    println!("file download");
    let files= files.clone();
    let title= String::new();
    let mut path= String::new();
    let mut search= String::new();
    io::stdin().read_line(&mut search).unwrap();

    for i in files{
        io::stdin().read_line(&mut search).unwrap();
        let search= search.trim();
        let title= i.name.clone();
        if title == search {
            path= i.path.clone();
        }
    }
    
    (title, path)
}

pub fn upload() -> Files{
    println!("file upload: Entering");

    println!("File Name >>");
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let file_name= buf.trim();

    println!("File Volume.");
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let file_volum= buf.trim().parse::<f64>().unwrap();

    println!("Path");
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let file_path= buf.trim();

    Files { 
        name: file_name.to_string(), 
        volum: file_volum,
        path: file_path.to_string(), 
    }
}

pub fn delete(files: Vec<Files>) -> Vec<Files>{
    println!("file delete");
    let mut v= files;

    let mut index= 0;
    let mut title= String::new();
    io::stdin().read_line(&mut title).unwrap();
    for (i, j) in v.iter().enumerate(){
        if title == j.name{
            index= i;
        }
    }
    
    v.remove(index);
    
    return v;
}

pub fn inputing() -> usize {
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    
    buf.trim().parse::<usize>().unwrap()
}