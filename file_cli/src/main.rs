use std::{io::{self, Write}, env::{self}, fs::{File, self}};

struct Arguments{
    filename: String,
    path    : String,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str>{
        let mut filename= String::new();

        if args.len() < 2{
            return Err("not enought arguments");
        } else if args.len() > 4{
            return Err("too many");
        }

        let option= args[1].clone();
        if option.contains("-h") {
            let menu= "-h   -help
-m  -modifiy(rename)
-w  -write
-r  -read";
            println!("{}", menu);
        }

        if option.contains("-m") || option.contains("-modifiy"){
            filename= args[3];
            Arguments::modifiy(filename);
        }

        if option.contains("-r") || option.contains("-read"){
            // read 
            filename= args[3];
            Arguments::read(filename);
        }

        let argument= Arguments { filename, path };
        Ok(argument)
    }

    
    fn read(filename: String){
        let read = fs::read_to_string(filename).expect("faild to read");
        println!("{}", read);
    }
    
    fn reloacte(path1: String, path2: String){
        let seleted= path1;
        let relocating= path2;
        // 1. copy and delelte
        // 2. path1 -> path2
    }
    
    fn modifiy(filename: String){
        let mut file = File::create(filename).expect("faild to open");
        let sentens= b"Hello, world!";
        file.write_all(sentens).expect("faile to write");
    }
}

fn showString(){
    println!("## File Fixer! ##");
    
    let usage= "Usage: file-fixer -options file_name path";
    let menu= "-h   -help
-m  -modifiy(rename)
-w  -write
-r  -read";

    println!("{}", usage);
    println!("{}", menu);
}

fn main() {
    showString();

    let args: Vec<String>= env::args().collect();
    let program= args[0].clone();
    let file_name= args[2].clone();

    let arguments= Arguments::new(&args).expect("Failed to load a file");
    // let path

    // println!("{:?}", args);
}
