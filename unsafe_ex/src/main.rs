// https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html

use std::slice;

unsafe fn dangerous(r1: *const i32, r2: *mut i32){
    println!("r1 {}", *r1);
    println!("r2 {}", *r2);
}

fn splite_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]){
    let len= slice.len();
    let ptr= slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid);
        )
    }
}

extern "C"{
    fn abs(input: i32) -> i32;
}

// Accessing or Modifying a Mutable Static Variable
static mut COUNTER: u32= 0;
fn add_to_count(inc: u32) {
    unsafe{
        COUNTER += inc;
    }
}

fn main() {
    let mut num= 5;

    let r1= &num as *const i32;
    let r2= &mut num as *mut i32;
    
    unsafe{
        dangerous(r1, r2);
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // Accessing or Modifying a Mutable Static Variable
    add_to_count(3);
    unsafe{
        println!("COUNTER: {}", COUNTER);
    }
}
