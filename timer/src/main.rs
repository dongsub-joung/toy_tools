use std::{os::unix::thread, fs::File};

fn main() {
    let ring= "ring.mp3";
    let mut show= 180;
    loop {
        println!("{}", show);
        
        // thread::wait_millisecond(1000);
        show -= 1;

        if show == 0 {
            // play a ring;
            File::open(ring).expect("Not found file");
            
        } 
    }
}
