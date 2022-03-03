use soloud::*;
use std::{thread, time};

fn main() {
    let mut show= 5;
    loop {
        println!("{}", show);
        
        let millis= time::Duration::from_millis(1000);
        thread::sleep(millis);

        show -= 1;

        if show == 0 {
            // play a ring;
            let sl = Soloud::default().unwrap();
            let mut wav = audio::Wav::default();
            wav.load_mem(include_bytes!("../audi.wav")).unwrap();
            sl.play(&wav);
            while sl.voice_count() > 0 {
                thread::sleep(time::Duration::from_millis(100));
            }

            break;
        } 
    }
}
